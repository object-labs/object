use std::net::SocketAddr;
use std::sync::Arc;

use axum::{
    Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::{Html as HtmlBody, Json as JsonBody},
    routing::{get, post},
};
use chrono::Utc;
use object_api::{
    FundingState, Market, Order, OrderBook, OrderInput, OrderLevel, OrderType, PerpetualPosition,
};
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::{EnvFilter, fmt};

const API_BASE_PATH: &str = "/api";
const CURRENT_API_VERSION: &str = "v1";
const SUPPORTED_API_VERSIONS: [&str; 1] = [CURRENT_API_VERSION];

type SharedState = Arc<RwLock<AppState>>;
type JsonError = (StatusCode, JsonBody<ApiError>);
type JsonSuccess<T> = JsonBody<ApiResponse<T>>;

#[derive(Clone, Serialize, Deserialize)]
struct ApiResponse<T: Serialize> {
    data: T,
}

#[derive(Serialize)]
struct ApiError {
    error: String,
}

#[derive(Default)]
struct AppState {
    markets: Vec<Market>,
    orders: Vec<Order>,
    positions: Vec<PerpetualPosition>,
    funding: Vec<FundingState>,
    next_order_id: u64,
}

#[derive(Deserialize)]
struct ListOrdersQuery {
    market: Option<String>,
}

#[tokio::main]
async fn main() {
    fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .pretty()
        .with_target(true)
        .init();

    let state = Arc::new(RwLock::new(seed_state()));
    let mut api_router = Router::new().route("/", get(api_versions));
    for version in SUPPORTED_API_VERSIONS.iter() {
        api_router = api_router.nest(
            &format!("/{version}"),
            build_version_router(version, state.clone()),
        );
    }

    let app = Router::new()
        .route("/", get(openapi_web_app))
        .nest(API_BASE_PATH, api_router)
        .layer(TraceLayer::new_for_http())
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        );

    let addr: SocketAddr = "0.0.0.0:3000".parse().expect("valid socket");
    info!("Object API server listening on {}", addr);
    axum::serve(
        tokio::net::TcpListener::bind(addr).await.expect("bind"),
        app,
    )
    .await
    .expect("server");
}

fn build_version_router(_version: &str, state: SharedState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/openapi.json", get(openapi_spec))
        .route("/markets", get(list_markets))
        .route("/markets/:market/orderbook", get(order_book))
        .route("/orders", post(create_order).get(list_orders))
        .route("/markets/:market/funding", get(funding_state))
        .route("/markets/:market/positions", get(positions))
        .with_state(state)
}

fn now_ms() -> u64 {
    Utc::now().timestamp_millis().try_into().unwrap_or(0)
}

fn seed_state() -> AppState {
    AppState {
        markets: vec![
            Market {
                symbol: "BTC-USDT".to_string(),
                market_type: "spot".to_string(),
                base_asset: "BTC".to_string(),
                quote_asset: "USDT".to_string(),
            },
            Market {
                symbol: "ETH-USDT".to_string(),
                market_type: "spot".to_string(),
                base_asset: "ETH".to_string(),
                quote_asset: "USDT".to_string(),
            },
            Market {
                symbol: "ETH-PERP".to_string(),
                market_type: "perpetual".to_string(),
                base_asset: "ETH".to_string(),
                quote_asset: "USDC".to_string(),
            },
        ],
        orders: vec![],
        positions: vec![PerpetualPosition {
            market: "ETH-PERP".to_string(),
            account: "demo_user".to_string(),
            size: 1.25,
            entry_price: 2600.0,
            unrealized_pnl: 14.2,
            margin_used: 2200.0,
            mark_price: 2611.4,
        }],
        funding: vec![FundingState {
            market: "ETH-PERP".to_string(),
            current_funding_rate: 0.00015,
            next_funding_interval_ms: 7_200_000,
        }],
        next_order_id: 1_000,
    }
}

fn bad_request(message: &str) -> JsonError {
    (
        StatusCode::BAD_REQUEST,
        JsonBody(ApiError {
            error: message.to_string(),
        }),
    )
}

fn not_found(message: &str) -> JsonError {
    (
        StatusCode::NOT_FOUND,
        JsonBody(ApiError {
            error: message.to_string(),
        }),
    )
}

async fn health() -> JsonSuccess<String> {
    JsonBody(ApiResponse {
        data: "ok".to_string(),
    })
}

async fn api_versions() -> JsonBody<ApiResponse<Vec<String>>> {
    JsonBody(ApiResponse {
        data: SUPPORTED_API_VERSIONS
            .iter()
            .map(|version| version.to_string())
            .collect(),
    })
}

async fn openapi_spec() -> JsonBody<serde_json::Value> {
    let spec = include_str!("../../../api/openapi/object-openapi.json");
    let spec: serde_json::Value =
        serde_json::from_str(spec).expect("valid object openapi document");
    JsonBody(spec)
}

async fn list_markets(State(state): State<SharedState>) -> JsonSuccess<Vec<Market>> {
    let state = state.read().await;
    JsonBody(ApiResponse {
        data: state.markets.clone(),
    })
}

async fn list_orders(
    State(state): State<SharedState>,
    Query(query): Query<ListOrdersQuery>,
) -> JsonSuccess<Vec<Order>> {
    let state = state.read().await;
    let items = match query.market {
        Some(market) => state
            .orders
            .iter()
            .filter(|order| order.market == market)
            .cloned()
            .collect(),
        None => state.orders.clone(),
    };
    JsonBody(ApiResponse { data: items })
}

async fn create_order(
    State(state): State<SharedState>,
    JsonBody(payload): JsonBody<OrderInput>,
) -> Result<(StatusCode, JsonSuccess<Order>), JsonError> {
    if payload.size <= 0.0 {
        return Err(bad_request("size must be greater than zero"));
    }

    if matches!(payload.order_type, OrderType::Limit) && payload.price.is_none() {
        return Err(bad_request("limit order requires a price"));
    }

    let mut state = state.write().await;
    let market_exists = state.markets.iter().any(|m| m.symbol == payload.market);
    if !market_exists {
        return Err(not_found("market not found"));
    }

    let order_id = format!("ord_{}", state.next_order_id);
    state.next_order_id = state.next_order_id.saturating_add(1);

    let order = Order {
        id: order_id,
        status: "open".to_string(),
        market: payload.market,
        side: payload.side,
        order_type: payload.order_type,
        size: payload.size,
        price: payload.price,
        reduce_only: payload.reduce_only,
        account: payload.account,
        created_at_ms: now_ms(),
    };
    state.orders.push(order.clone());
    Ok((StatusCode::CREATED, JsonBody(ApiResponse { data: order })))
}

async fn positions(
    State(state): State<SharedState>,
    Path(market): Path<String>,
) -> JsonSuccess<Vec<PerpetualPosition>> {
    let state = state.read().await;
    let positions = state
        .positions
        .iter()
        .filter(|position| position.market == market)
        .cloned()
        .collect();
    JsonBody(ApiResponse { data: positions })
}

async fn funding_state(
    State(state): State<SharedState>,
    Path(market): Path<String>,
) -> Result<JsonSuccess<FundingState>, JsonError> {
    let state = state.read().await;
    let funding = state
        .funding
        .iter()
        .find(|funding| funding.market == market)
        .cloned();

    funding
        .map(|data| JsonBody(ApiResponse { data }))
        .ok_or_else(|| not_found("funding not available for market"))
}

async fn order_book(
    State(state): State<SharedState>,
    Path(market): Path<String>,
) -> Result<JsonSuccess<OrderBook>, JsonError> {
    let state = state.read().await;
    if !state.markets.iter().any(|item| item.symbol == market) {
        return Err(not_found("market not found"));
    }

    let mut bids = vec![
        OrderLevel {
            price: 50000.0,
            size: 0.25,
        },
        OrderLevel {
            price: 49950.0,
            size: 0.4,
        },
    ];
    let mut asks = vec![
        OrderLevel {
            price: 50050.0,
            size: 0.18,
        },
        OrderLevel {
            price: 50100.0,
            size: 0.8,
        },
    ];
    if market.starts_with("ETH") {
        bids[0].price = 2600.0;
        asks[0].price = 2605.0;
        bids[1].price = 2597.0;
        asks[1].price = 2612.0;
    }

    Ok(JsonBody(ApiResponse {
        data: OrderBook {
            market,
            bids,
            asks,
            updated_at_ms: now_ms(),
        },
    }))
}

async fn openapi_web_app() -> HtmlBody<String> {
    const OPENAPI_UI_TEMPLATE: &str = r#"<!doctype html>
<html lang="en">
  <head>
    <meta charset="utf-8" />
    <meta name="viewport" content="width=device-width, initial-scale=1" />
    <title>Object API Explorer</title>
    <link
      rel="stylesheet"
      href="https://cdn.jsdelivr.net/npm/swagger-ui-dist@5/swagger-ui.css"
    />
    <style>
      html, body {
        margin: 0;
        padding: 0;
      }
      #swagger-ui {
        min-height: 100vh;
      }
    </style>
    </head>
  <body>
    <div id="swagger-ui"></div>
    <script src="https://cdn.jsdelivr.net/npm/swagger-ui-dist@5/swagger-ui-bundle.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/swagger-ui-dist@5/swagger-ui-standalone-preset.js"></script>
    <script>
      window.addEventListener('load', function () {
        const openapiUrl = new URL('{openapi_url}', window.location.href).toString();
        window.ui = SwaggerUIBundle({
          url: openapiUrl,
          dom_id: '#swagger-ui',
          presets: [SwaggerUIBundle.presets.apis, SwaggerUIStandalonePreset],
          layout: 'BaseLayout',
          tryItOutEnabled: true,
        });
      });
    </script>
  </body>
    </html>"#;
    let openapi_url = format!("{API_BASE_PATH}/{CURRENT_API_VERSION}/openapi.json");
    let html = OPENAPI_UI_TEMPLATE.replace("{openapi_url}", &openapi_url);

    HtmlBody(html)
}

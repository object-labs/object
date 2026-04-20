import type {
  ApiResponse,
  ErrorResponse,
  FundingResponse,
  Market,
  MarketListResponse,
  OrderBookResponse,
  OrderInput,
  Order,
  OrderListResponse,
  OrderResponse,
  OrderType,
  OrderSide,
  PerpetualPosition,
  PositionsResponse,
} from "./types";

const API_BASE = import.meta.env.VITE_API_BASE_URL ?? "/api";

async function toJson<T>(response: Response): Promise<T> {
  if (!response.ok) {
    const body = (await response.json().catch(() => null)) as ErrorResponse | null;
    throw new Error(body?.error ?? `Request failed (${response.status})`);
  }
  return response.json() as Promise<T>;
}

export const getHealth = async (): Promise<ApiResponse<string>> =>
  toJson(await fetch(`${API_BASE}/health`));

export const listMarkets = async (): Promise<Market[]> =>
  toJson<MarketListResponse>(await fetch(`${API_BASE}/markets`)).then((r) => r.data);

export const getOrderBook = async (market: string): Promise<OrderBookResponse["data"]> =>
  toJson<OrderBookResponse>(await fetch(`${API_BASE}/markets/${market}/orderbook`)).then((r) => r.data);

export const getFundingState = async (market: string): Promise<FundingResponse["data"]> =>
  toJson<FundingResponse>(await fetch(`${API_BASE}/markets/${market}/funding`)).then((r) => r.data);

export const getPositions = async (market: string): Promise<PerpetualPosition[]> =>
  toJson<PositionsResponse>(await fetch(`${API_BASE}/markets/${market}/positions`)).then((r) => r.data);

export const listOrders = async (market?: string): Promise<Order[]> =>
  toJson<OrderListResponse>(
    await fetch(`${API_BASE}/orders${market ? `?market=${market}` : ""}`),
  ).then((r) => r.data);

export const createOrder = async (input: {
  market: string;
  side: OrderSide;
  order_type: OrderType;
  size: number;
  price?: number | null;
  reduce_only: boolean;
  account: string;
}): Promise<OrderResponse["data"]> =>
  toJson<OrderResponse>(
    await fetch(`${API_BASE}/orders`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify(input),
    }),
  ).then((r) => r.data);

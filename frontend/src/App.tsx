import { ChangeEvent, FormEvent, useMemo, useState } from "react";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import type { Market, Order, OrderSide, OrderType } from "./api/types";
import {
  createOrder,
  getFundingState,
  getOrderBook,
  getPositions,
  listMarkets,
  listOrders,
  getHealth,
} from "./api/client";

type MarketFilters = "all" | "spot" | "perpetual";

const defaultAccount = "demo_user";
const defaultOrder = {
  side: "buy" as OrderSide,
  order_type: "limit" as OrderType,
  size: 0.1,
  reduce_only: false,
};

function Section({ title, children }: { title: string; children: React.ReactNode }) {
  return (
    <section className="panel">
      <h2>{title}</h2>
      {children}
    </section>
  );
}

export default function App() {
  const queryClient = useQueryClient();

  const { data: health } = useQuery({
    queryKey: ["health"],
    queryFn: getHealth,
    staleTime: 5_000,
  });

  const [filter, setFilter] = useState<MarketFilters>("all");
  const { data: markets = [] } = useQuery({
    queryKey: ["markets"],
    queryFn: listMarkets,
    staleTime: 5_000,
  });

  const filteredMarkets = useMemo(() => {
    if (filter === "all") return markets;
    return markets.filter((market) => market.market_type === filter);
  }, [markets, filter]);

  const [activeMarket, setActiveMarket] = useState("");
  const selectedMarket = markets.find((m) => m.symbol === activeMarket) ?? null;

  const { data: book } = useQuery({
    queryKey: ["orderbook", activeMarket],
    queryFn: () => getOrderBook(activeMarket),
    enabled: Boolean(activeMarket),
  });

  const { data: positions = [] } = useQuery({
    queryKey: ["positions", activeMarket],
    queryFn: () => getPositions(activeMarket),
    enabled: Boolean(activeMarket),
  });

  const { data: funding } = useQuery({
    queryKey: ["funding", activeMarket],
    queryFn: () => getFundingState(activeMarket),
    enabled: Boolean(activeMarket),
  });

  const { data: orders = [] } = useQuery({
    queryKey: ["orders", activeMarket],
    queryFn: () => listOrders(activeMarket || undefined),
    enabled: Boolean(activeMarket),
  });

  const { mutate, isPending, error } = useMutation({
    mutationFn: (input: {
      market: string;
      side: OrderSide;
      order_type: OrderType;
      size: number;
      price?: number | null;
      reduce_only: boolean;
      account: string;
    }) => createOrder(input),
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ["orders", activeMarket] });
    },
  });

  const [orderForm, setOrderForm] = useState({
    ...defaultOrder,
    market: "",
    price: "",
  });

  const onSubmit = (event: FormEvent) => {
    event.preventDefault();
    if (!activeMarket) {
      return;
    }

    mutate({
      market: activeMarket,
      side: orderForm.side,
      order_type: orderForm.order_type,
      size: Number(orderForm.size),
      price: orderForm.order_type === "limit" ? Number(orderForm.price) : null,
      reduce_only: orderForm.reduce_only,
      account: defaultAccount,
    });
  };

  return (
    <main className="app-shell">
      <header className="app-header">
        <h1>Object Exchange</h1>
        <p>
          Health:{" "}
          <strong>
            {health?.data ? "connected" : "connecting…"}
          </strong>
        </p>
      </header>

      <section className="controls">
        <label>
          Filter
          <select value={filter} onChange={(event: ChangeEvent<HTMLSelectElement>) => setFilter(event.target.value as MarketFilters)}>
            <option value="all">All markets</option>
            <option value="spot">Spot</option>
            <option value="perpetual">Perpetual</option>
          </select>
        </label>

        <label>
          Active market
          <select
            value={activeMarket}
            onChange={(event: ChangeEvent<HTMLSelectElement>) => setActiveMarket(event.target.value)}
          >
            <option value="" disabled>
              Pick a market
            </option>
            {filteredMarkets.map((market) => (
              <option key={market.symbol} value={market.symbol}>
                {market.symbol}
              </option>
            ))}
          </select>
        </label>
      </section>

      <div className="grid">
        <Section title="Markets">
          <ul>
            {filteredMarkets.map((market) => (
              <MarketRow
                key={market.symbol}
                market={market}
                active={market.symbol === activeMarket}
                onActivate={() => setActiveMarket(market.symbol)}
              />
            ))}
          </ul>
        </Section>

        <Section title="Orderbook">
          {!book ? (
            <p>Choose a market to view depth.</p>
          ) : (
            <div className="book">
              <div>
                <h3>Bids</h3>
                {book.bids.map((level, index) => (
                  <div key={`${index}-bid`} className="line bid">
                    <span>{level.price.toFixed(2)}</span>
                    <span>{level.size.toFixed(4)}</span>
                  </div>
                ))}
              </div>
              <div>
                <h3>Asks</h3>
                {book.asks.map((level, index) => (
                  <div key={`${index}-ask`} className="line ask">
                    <span>{level.price.toFixed(2)}</span>
                    <span>{level.size.toFixed(4)}</span>
                  </div>
                ))}
              </div>
            </div>
          )}
        </Section>

        <Section title="Order Entry">
          <form onSubmit={onSubmit} className="order-form">
            <label>
              Side
              <select
                value={orderForm.side}
                onChange={(event: ChangeEvent<HTMLSelectElement>) =>
                  setOrderForm((prev) => ({ ...prev, side: event.target.value as OrderSide }))
                }
              >
                <option value="buy">Buy</option>
                <option value="sell">Sell</option>
              </select>
            </label>
            <label>
              Type
              <select
                value={orderForm.order_type}
                onChange={(event: ChangeEvent<HTMLSelectElement>) =>
                  setOrderForm((prev) => ({
                    ...prev,
                    order_type: event.target.value as OrderType,
                  }))
                }
              >
                <option value="limit">Limit</option>
                <option value="market">Market</option>
              </select>
            </label>
            <label>
              Size
              <input
                type="number"
                min={0.0001}
                step={0.0001}
                value={orderForm.size}
                onChange={(event) =>
                  setOrderForm((prev) => ({ ...prev, size: Number(event.target.value) }))
                }
              />
            </label>
            <label>
              Limit Price
              <input
                type="number"
                min={0}
                step={0.01}
                value={orderForm.price}
                disabled={orderForm.order_type === "market"}
                onChange={(event) =>
                  setOrderForm((prev) => ({ ...prev, price: event.target.value }))
                }
              />
            </label>
            <label>
              <input
                type="checkbox"
                checked={orderForm.reduce_only}
                onChange={(event) =>
                  setOrderForm((prev) => ({ ...prev, reduce_only: event.target.checked }))
                }
              />
              Reduce-only
            </label>
            <button type="submit" disabled={isPending || !activeMarket}>
              {isPending ? "Submitting..." : "Submit Order"}
            </button>
            {error ? <p className="error">{error.message}</p> : null}
          </form>
        </Section>

        <Section title="Open Orders">
          <ul className="compact">
            {orders.length === 0 ? <li>No orders</li> : null}
            {orders.map((order: Order) => (
              <li key={order.id}>
                {order.market} {order.side} {order.order_type} {order.size} @ {order.price ?? "market"} — {order.status}
              </li>
            ))}
          </ul>
        </Section>

        {selectedMarket?.market_type === "perpetual" ? (
          <>
            <Section title="Perpetual Funding">
              {funding ? (
                <p>Rate: {funding.current_funding_rate.toFixed(6)}</p>
              ) : (
                <p>No funding state</p>
              )}
            </Section>
            <Section title="Perpetual Positions">
              {positions.length === 0 ? <p>No active position.</p> : null}
              {positions.map((position) => (
                <div key={`${position.market}-${position.account}`} className="position">
                  <span>{position.account}</span>
                  <span>{position.size}</span>
                  <span>{position.unrealized_pnl.toFixed(2)} pnl</span>
                </div>
              ))}
            </Section>
          </>
        ) : null}
      </div>
    </main>
  );
}

function MarketRow({
  market,
  active,
  onActivate,
}: {
  market: Market;
  active: boolean;
  onActivate: () => void;
}) {
  return (
    <li className={`market ${active ? "active" : ""}`} onClick={onActivate} role="button">
      <strong>{market.symbol}</strong>
      <span>{market.market_type}</span>
      <span>{market.base_asset}/{market.quote_asset}</span>
    </li>
  );
}

import type { components } from "./openapi-types";

export type ApiResponse<T> = {
  data: T;
};

export type ErrorResponse = components["schemas"]["ErrorResponse"];

export type Market = components["schemas"]["Market"];
export type OrderSide = components["schemas"]["OrderSide"];
export type OrderType = components["schemas"]["OrderType"];
export type OrderInput = components["schemas"]["OrderInput"];
export type Order = components["schemas"]["Order"];
export type OrderLevel = components["schemas"]["OrderLevel"];
export type OrderBook = components["schemas"]["OrderBook"];
export type PerpetualPosition = components["schemas"]["PerpetualPosition"];
export type FundingState = components["schemas"]["FundingState"];

export type MarketListResponse = components["schemas"]["MarketListResponse"];
export type OrderListResponse = components["schemas"]["OrderListResponse"];
export type OrderResponse = components["schemas"]["OrderResponse"];
export type OrderBookResponse = components["schemas"]["OrderBookResponse"];
export type PositionsResponse = components["schemas"]["PositionsResponse"];
export type FundingResponse = components["schemas"]["FundingResponse"];


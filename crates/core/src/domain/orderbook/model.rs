use std::time;

use rust_decimal::Decimal;

pub struct Level {
    pub order_id: i32,
    pub price: Decimal,
    pub quantity: Decimal,
    pub created_at: time::Instant,
}

#[derive(Debug)]
pub enum EventType {
    Trade,
    Kline,
    Orderbook,
    Ticker,
}

#[derive(Debug)]
pub struct TradeStreamData {
    pub event_type: EventType,
    pub timestamp: time::Instant,
    pub symbol: String,
    pub trade_id: i32,
    pub price: Decimal,
    pub quantity: Decimal,
    pub is_buyer_maker: bool,
}

pub struct KlineStreamData {
    pub event_type: EventType,
    pub timestamp: time::Instant,
    pub symbol: String,
    pub start_time: time::Instant,
    pub close_time: time::Instant,
    pub interval: String,
    pub first_trade_id: i32,
    pub last_trade_id: i32,
    pub open_price: Decimal,
    pub close_price: Decimal,
    pub low_price: Decimal,
    pub high_price: Decimal,
    pub is_closed: bool,
    pub trade_count: i32,
    pub base_asset_volume: Decimal,
    pub quote_asset_volume: Decimal,
    pub taker_base_asset_volume: Decimal,
    pub taker_quote_asset_volumn: Decimal,
}

pub struct PartialDepthOrderbookStreamData {
    pub event_type: EventType,
    pub timestamp: EventType,
    pub symbol: String,
    pub bids: Vec<(Decimal, Decimal)>,
    pub asks: Vec<(Decimal, Decimal)>,
}

// 24 Hour Rolling Window Ticker
pub struct TickerStreamData {
    pub event_type: EventType,
    pub timestamp: EventType,
    pub symbol: String,
    pub price_change: Decimal,
    pub price_change_percent: Decimal,
    pub weighted_average_price: Decimal,
    pub last_prev_trade_price: Decimal,
    pub last_price: Decimal,
    pub last_quantity: Decimal,
    pub best_bid_price: Decimal,
    pub best_bid_quantity: Decimal,
    pub best_ask_price: Decimal,
    pub best_ask_quantity: Decimal,
    pub open_price: Decimal,
    pub close_price: Decimal,
    pub high_price: Decimal,
    pub low_price: Decimal,
    pub base_asset_volume_traded: Decimal,
    pub quote_asset_volume_traded: Decimal,
    pub window_open_time: time::Instant,
    pub window_close_time: time::Instant,
}

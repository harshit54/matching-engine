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
    event_type: EventType,
    timestamp: time::Instant,
    symbol: String,
    start_time: time::Instant,
    close_time: time::Instant,
    interval: String,
    first_trade_id: i32,
    last_trade_id: i32,
    open_price: Decimal,
    close_price: Decimal,
    low_price: Decimal,
    high_price: Decimal,
    is_closed: bool,
    trade_count: i32,
    base_asset_volume: Decimal,
    quote_asset_volume: Decimal,
    taker_base_asset_volume: Decimal,
    taker_quote_asset_volumn: Decimal,
}

pub struct PartialDepthOrderbookStreamData {
    event_type: EventType,
    timestamp: EventType,
    symbol: String,
    bids: Vec<(Decimal, Decimal)>,
    asks: Vec<(Decimal, Decimal)>,
}

// 24 Hour Rolling Window Ticker
pub struct TickerStreamData {
    event_type: EventType,
    timestamp: EventType,
    symbol: String,
    price_change: Decimal,
    price_change_percent: Decimal,
    weighted_average_price: Decimal,
    last_prev_trade_price: Decimal,
    last_price: Decimal,
    last_quantity: Decimal,
    best_bid_price: Decimal,
    best_bid_quantity: Decimal,
    best_ask_price: Decimal,
    best_ask_quantity: Decimal,
    open_price: Decimal,
    close_price: Decimal,
    high_price: Decimal,
    low_price: Decimal,
    base_asset_volume_traded: Decimal,
    quote_asset_volume_traded: Decimal,
    window_open_time: time::Instant,
    window_close_time: time::Instant,
}

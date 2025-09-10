use tokio::sync::broadcast;

use crate::domain::{
    error::ServiceError,
    order::Order,
    orderbook::model::{
        KlineStreamData, PartialDepthOrderbookStreamData, TickerStreamData, TradeStreamData,
    },
};

pub trait Service {
    fn add_order(&self, username: String) -> Result<Order, ServiceError>;
    fn get_order(&self, id: i32) -> Result<Order, ServiceError>;

    // Private Data Streams // TODO: Add broker domain object later.
    fn get_private_trade_stream(&self, instrument_id: i32);
    fn get_private_order_stream(&self, instrument_id: i32);

    // Public Data Streams
    fn get_trade_stream(&self, instrument_id: i32) -> broadcast::Receiver<TradeStreamData>;
    fn get_kline_stream(&self, instrument_id: i32) -> broadcast::Receiver<KlineStreamData>;
    fn get_orderbook_stream(
        &self,
        instrument_id: i32,
    ) -> broadcast::Receiver<PartialDepthOrderbookStreamData>;
    fn get_ticker_stream(&self, instrument_id: i32) -> broadcast::Receiver<TickerStreamData>;
}

use std::{
    cmp::{Ordering, Reverse},
    collections::{BTreeMap, LinkedList},
    error::Error,
    fmt::Binary,
    iter::Rev,
    time,
};

use log::{error, info};
use rust_decimal::Decimal;
use tokio::sync::broadcast::{self, error::SendError};

use crate::{
    domain::{
        error::ServiceError,
        order::{
            self, Order,
            model::{Side, State, Type},
        },
        orderbook::model::{
            EventType, KlineStreamData, Level, PartialDepthOrderbookStreamData, TickerStreamData,
            TradeStreamData,
        },
    },
    repo::{error::RepoError, order::OrderRepo, trade::TradeRepo},
};

pub trait Service {
    fn add_order(&mut self, order: &Order) -> Result<(), ServiceError>;

    // Private Data Streams // TODO: Add broker domain object later.
    // fn get_private_trade_stream(&self, instrument_id: i32);
    // fn get_private_order_stream(&self, instrument_id: i32);

    // Public Data Streams
    fn get_trade_stream(&self) -> &broadcast::Receiver<TradeStreamData>;
    // fn get_kline_stream(&self, instrument_id: i32) -> broadcast::Receiver<KlineStreamData>;
    // fn get_orderbook_stream(
    //     &self,
    //     instrument_id: i32,
    // ) -> broadcast::Receiver<PartialDepthOrderbookStreamData>;
    // fn get_ticker_stream(&self, instrument_id: i32) -> broadcast::Receiver<TickerStreamData>;
}

pub struct Channel<T> {
    sender: broadcast::Sender<T>,
    receiver: broadcast::Receiver<T>,
}

pub struct ServiceImpl<O: OrderRepo, T: TradeRepo> {
    instrument_symbol: String,

    order_repo: O,
    trade_repo: T,

    bids: BTreeMap<Decimal, LinkedList<Level>>,
    asks: BTreeMap<Reverse<Decimal>, LinkedList<Level>>,

    trade_stream: Channel<TradeStreamData>,
    // kline_stream: Channel<KlineStreamData>,
    // orderbook_stream: Channel<PartialDepthOrderbookStreamData>,
    // ticker_stream: Channel<TickerStreamData>,
}

impl<O: OrderRepo, T: TradeRepo> ServiceImpl<O, T> {
    fn place_limit_order(&mut self, order: &Order) -> Result<(), ServiceError> {
        let price = order.price.unwrap();
        let ob_level = Level {
            order_id: order.id,
            price: price,
            quantity: order.quantity,
            created_at: time::Instant::now(),
        };

        match order.order_side {
            Side::Buy => match self.bids.get_mut(&price) {
                Some(ll) => ll.push_back(ob_level),
                None => {
                    let mut ll: LinkedList<Level> = LinkedList::new();
                    ll.push_back(ob_level);
                    self.bids.insert(price, ll);
                }
            },

            Side::Sell => match self.asks.get_mut(&Reverse(price)) {
                Some(ll) => ll.push_back(ob_level),
                None => {
                    let mut ll: LinkedList<Level> = LinkedList::new();
                    ll.push_back(ob_level);
                    self.asks.insert(Reverse(price), ll);
                }
            },
        }

        self.execute_matches();

        Ok(())
    }

    fn execute_matches(&mut self) {
        let mut bids_iter = self.bids.iter_mut();
        let mut asks_iter = self.asks.iter_mut();

        loop {
            let curr_bid = bids_iter.next();
            if curr_bid.is_none() {
                return;
            }

            let curr_ask = asks_iter.next();
            if curr_ask.is_none() {
                return;
            }

            let (bid_price, bid_ll) = curr_bid.unwrap();
            let (ask_price_rev, ask_ll) = curr_ask.unwrap();
            let ask_price = &ask_price_rev.0;

            if bid_price < ask_price {
                info!("no matches possible - spread {:?}", bid_price - ask_price);
                return;
            }

            loop {
                let bid_level: &mut Level;
                let ask_level: &mut Level;

                match bid_ll.front_mut() {
                    Some(ll) => bid_level = ll,
                    None => break,
                }

                match ask_ll.front_mut() {
                    Some(ll) => ask_level = ll,
                    None => break,
                }

                let is_buyer_maker = bid_level.created_at < ask_level.created_at;
                match ask_level.quantity.cmp(&bid_level.quantity) {
                    // Ask cannot fill complete bid.
                    // Execute trade, remove ask, decrease bid_qty.
                    Ordering::Less => {
                        let res: Result<(), ExecTradeError> = Self::exec_trade(
                            ExecTradeVariant::AskFilled,
                            &self.instrument_symbol,
                            &mut self.order_repo,
                            &mut self.trade_repo,
                            &mut self.trade_stream.sender,
                            is_buyer_maker,
                            bid_level.order_id,
                            ask_level.order_id,
                            ask_price,
                            &ask_level.quantity,
                        );

                        match res {
                            Ok(_) => {
                                bid_level.quantity -= ask_level.quantity;
                                ask_ll.pop_front();
                                continue;
                            }
                            Err(err) => {
                                error!("orderbook: stopping trade execution: {:?}", err);
                                return;
                            }
                        }
                    }
                    // Ask exactly fills the bid.
                    // Execute trade, remove ask, remove bid.
                    Ordering::Equal => {
                        let res = Self::exec_trade(
                            ExecTradeVariant::BothSidesFilled,
                            &self.instrument_symbol,
                            &mut self.order_repo,
                            &mut self.trade_repo,
                            &mut self.trade_stream.sender,
                            is_buyer_maker,
                            bid_level.order_id,
                            ask_level.order_id,
                            ask_price,
                            &bid_level.quantity,
                        );

                        match res {
                            Ok(_) => {
                                ask_ll.pop_front();
                                bid_ll.pop_front();
                                continue;
                            }
                            Err(err) => {
                                error!("orderbook: stopping trade execution: {:?}", err);
                                return;
                            }
                        }
                    }
                    // Ask fills complete bid with leftover.
                    // Execute trade, decreae ask_qty, remove bid.
                    Ordering::Greater => {
                        let res = Self::exec_trade(
                            ExecTradeVariant::BidFilled,
                            &self.instrument_symbol,
                            &mut self.order_repo,
                            &mut self.trade_repo,
                            &mut self.trade_stream.sender,
                            is_buyer_maker,
                            bid_level.order_id,
                            ask_level.order_id,
                            ask_price,
                            &ask_level.quantity,
                        );

                        match res {
                            Ok(_) => {
                                ask_level.quantity -= bid_level.quantity;
                                bid_ll.pop_front();
                                continue;
                            }
                            Err(err) => {
                                error!("orderbook: stopping trade execution: {:?}", err);
                                return;
                            }
                        }
                    }
                }
            }
        }
    }

    fn exec_trade(
        variant: ExecTradeVariant,
        symbol: &String,
        order_repo: &mut O,
        trade_repo: &mut T,
        trade_stream: &mut broadcast::Sender<TradeStreamData>,
        is_buyer_maker: bool,
        buy_order_id: i32,
        sell_order_id: i32,
        price: &Decimal,
        qty: &Decimal,
    ) -> Result<(), ExecTradeError> {
        let buy_state: order::model::State;
        let sell_state: order::model::State;

        match variant {
            ExecTradeVariant::BidFilled => {
                buy_state = State::Filled;
                sell_state = State::PartiallyFilled;
            }
            ExecTradeVariant::AskFilled => {
                buy_state = State::PartiallyFilled;
                sell_state = State::Filled;
            }
            ExecTradeVariant::BothSidesFilled => {
                buy_state = State::Filled;
                sell_state = State::Filled;
            }
        }

        let order_res = order_repo.update_order_state(buy_order_id, buy_state);
        match order_res {
            Ok(_) => {}
            Err(err) => {
                error!("orderbook: buy order repo error: {:?}", err);
            }
        }

        let order_res = order_repo.update_order_state(sell_order_id, sell_state);
        match order_res {
            Ok(_) => {}
            Err(err) => {
                error!("orderbook: sell order repo error: {:?}", err);
            }
        }

        let trade_res = trade_repo.create_trade(sell_order_id, buy_order_id, price, qty);

        match trade_res {
            Ok(trade) => {
                match trade_stream.send(TradeStreamData {
                    event_type: EventType::Trade,
                    timestamp: trade.created_at,
                    symbol: symbol.clone(),
                    trade_id: trade.id,
                    price: trade.price,
                    quantity: trade.qty,
                    is_buyer_maker: is_buyer_maker,
                }) {
                    Ok(_) => Ok(()),
                    Err(err) => {
                        error!("orderbook: send error: {:?}", err);
                        Err(ExecTradeError::Send(err))
                    }
                }
            }
            Err(err) => {
                error!("orderbook: trade repo error: {:?}", err);
                Err(ExecTradeError::Repo(err))
            }
        }
    }

    fn place_market_order(&self, _order: &Order) -> Result<(), ServiceError> {
        todo!()
    }
}

#[derive(Debug)]
pub enum ExecTradeError {
    Repo(RepoError),
    Send(SendError<TradeStreamData>),
}

pub enum ExecTradeVariant {
    BidFilled,
    AskFilled,
    BothSidesFilled,
}

impl<O: OrderRepo, T: TradeRepo> Service for ServiceImpl<O, T> {
    fn add_order(&mut self, order: &Order) -> Result<(), ServiceError> {
        match order.order_type {
            Type::Market => self.place_market_order(order),

            Type::Limit => {
                if order.price.is_none() {
                    error!("limit order price is none: {:?}", order.price);
                    return Err(ServiceError::InternalError);
                }

                self.place_limit_order(order)
            }
        }
    }

    // fn get_private_trade_stream(&self, instrument_id: i32) {
    //     todo!()
    // }

    // fn get_private_order_stream(&self, instrument_id: i32) {
    //     todo!()
    // }

    fn get_trade_stream(&self) -> &broadcast::Receiver<TradeStreamData> {
        return &self.trade_stream.receiver;
    }

    // fn get_kline_stream(&self, instrument_id: i32) -> broadcast::Receiver<KlineStreamData> {
    //     todo!()
    // }

    // fn get_orderbook_stream(
    //     &self,
    //     instrument_id: i32,
    // ) -> broadcast::Receiver<PartialDepthOrderbookStreamData> {
    //     todo!()
    // }

    // fn get_ticker_stream(&self, instrument_id: i32) -> broadcast::Receiver<TickerStreamData> {
    //     todo!()
    // }
}

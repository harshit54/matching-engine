use crate::generator::Generator;
use crate::half_book::HalfBook;
use crate::types::{BookSide, OrderSide};
use std::fmt;

pub struct OrderBook {
    generator: Generator,
    bids: HalfBook,
    asks: HalfBook,
    ltp: u64,
}

impl fmt::Display for OrderBook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.bids.len() == 0 && self.asks.len() == 0 {
            return write!(f, "No Orders On Book");
        }

        write!(
            f,
            "Orders on Book\n Asks: {} LTP: {}\n Bids: {}",
            self.asks, self.ltp, self.bids,
        )
    }
}

impl OrderBook {
    pub fn new() -> OrderBook {
        let asks = HalfBook::new(BookSide::Asks);
        let bids = HalfBook::new(BookSide::Bids);
        let generator = Generator::new();

        OrderBook {
            generator,
            bids,
            asks,
            ltp: 0,
        }
    }

    pub fn remove_order(&mut self, oid: u64) -> bool {
        self.asks.remove_order(oid) || self.bids.remove_order(oid)
    }

    pub fn add_order(&mut self, side: OrderSide, price: u64, quantity: u64) -> u64 {
        let mut order = self.generator.new_order(side, price, quantity);
        let oid = order.id;

        let (reduce_hb, add_hb) = match side {
            OrderSide::Buy => (&mut self.asks, &mut self.bids),
            OrderSide::Sell => (&mut self.bids, &mut self.asks),
        };

        let trades = reduce_hb.match_and_reduce(&mut self.generator, &mut order);
        if trades.len() > 0 {
            println!("Trades: {trades:?}")
        }

        if order.quantity > 0 {
            add_hb.add_order(order)
        }
        oid
    }
}

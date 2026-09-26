use crate::Side;
use crate::generator::Generator;
use crate::half_book::HalfBook;
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
        let bids = HalfBook::new();
        let asks = HalfBook::new();
        let generator = Generator::new();

        OrderBook {
            generator,
            bids,
            asks,
            ltp: 0,
        }
    }

    pub fn add_order(&mut self, side: Side, price: u64, quantity: u64) -> u64 {
        let mut order = self.generator.new_order(side, price, quantity);
        let oid = order.id;

        match order.side {
            // For buy order, match and reduce on Asks, then add on Bids
            Side::Buy => {
                let trades = self.asks.match_and_reduce(&mut self.generator, &mut order);
                if trades.len() > 0 {
                    println!("Trades: {trades:?}")
                } else {
                    println!("No Trade")
                }

                if order.quantity > 0 {
                    self.bids.add_order(order)
                }
            }

            Side::Sell => {
                // For sell order, match and reduce on Bids, then add on Asks
                let trades = self.bids.match_and_reduce(&mut self.generator, &mut order);
                if trades.len() > 0 {
                    println!("Trades: {trades:?}")
                } else {
                    println!("No Trade")
                }

                if order.quantity > 0 {
                    self.asks.add_order(order)
                }
            }
        }

        oid
    }
}

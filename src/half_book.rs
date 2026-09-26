use crate::generator::Generator;
use crate::types::{BookSide, Order, Trade};
use std::cmp::Ordering;
use std::collections::btree_map::OccupiedEntry;
use std::collections::{BTreeMap, HashMap, VecDeque};
use std::fmt;

pub struct HalfBook {
    order_btree: BTreeMap<u64, VecDeque<Order>>,
    book_side: BookSide,
    order_map: HashMap<u64, u64>, // order_id -> price
}

impl fmt::Display for HalfBook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for level in self.order_btree.iter() {
            write!(f, "\n\t{}: {:?}", level.0, level.1)?;
        }

        write!(f, "\n")
    }
}

impl HalfBook {
    pub fn len(&self) -> usize {
        self.order_btree.len()
    }

    pub fn new(book_side: BookSide) -> Self {
        Self {
            order_btree: BTreeMap::new(),
            book_side,
            order_map: HashMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.order_map.insert(order.id, order.price);

        self.order_btree
            .entry(order.price)
            .or_default()
            .push_back(order)
    }

    pub fn remove_order(&mut self, oid: u64) -> bool {
        let price = match self.order_map.remove(&oid) {
            None => return false,
            Some(price) => price,
        };

        if let std::collections::btree_map::Entry::Occupied(mut entry) =
            self.order_btree.entry(price)
        {
            let orders = entry.get_mut();

            if let Some(index) = orders.iter().position(|o| o.id == oid) {
                if orders.len() == 1 {
                    entry.remove();
                    return true;
                }

                orders.remove(index);
                return true;
            }
        }

        false
    }

    fn get_best_price_level(
        &mut self,
    ) -> (
        Option<OccupiedEntry<'_, u64, VecDeque<Order>>>,
        &mut HashMap<u64, u64>,
    ) {
        // BTreeMap first_entry is the minimum entry.
        match self.book_side {
            BookSide::Asks => (self.order_btree.first_entry(), &mut self.order_map),
            BookSide::Bids => (self.order_btree.last_entry(), &mut self.order_map),
        }
    }

    fn will_trade(book_side: BookSide, book_price: &u64, in_price: &u64) -> bool {
        match book_side {
            BookSide::Asks => in_price >= book_price,
            BookSide::Bids => in_price <= book_price,
        }
    }

    pub fn match_and_reduce(
        &mut self,
        generator: &mut Generator,
        in_order: &mut Order,
    ) -> Vec<Trade> {
        let mut trades: Vec<Trade> = Vec::new();

        while in_order.quantity > 0 {
            match self.reduce_best_price(generator, in_order) {
                None => break,
                Some(matched_trades) => {
                    trades.extend(matched_trades);
                }
            }
        }

        trades
    }

    fn reduce_best_price(
        &mut self,
        generator: &mut Generator,
        in_order: &mut Order,
    ) -> Option<Vec<Trade>> {
        let book_side = self.book_side;
        let (pl, order_map) = self.get_best_price_level();
        if pl.is_none() {
            return None;
        }
        let price_level = pl?;

        let book_price = *price_level.key();
        let book_orders = price_level.into_mut();

        let mut matched_trades: Vec<Trade> = Vec::new();

        loop {
            match book_orders.front_mut() {
                None => break,

                Some(book_order) => {
                    if HalfBook::will_trade(book_side, &book_price, &in_order.price) {
                        match book_order.quantity.cmp(&in_order.quantity) {
                            // match and remove book order
                            Ordering::Less | Ordering::Equal => {
                                matched_trades.push(generator.new_trade(
                                    book_order.id,
                                    in_order.id,
                                    book_order.price,
                                    book_order.quantity,
                                ));

                                in_order.quantity -= book_order.quantity;
                                order_map.remove(&book_order.id);
                                book_orders.pop_front();
                            }

                            // match and update book order quantity
                            Ordering::Greater => {
                                matched_trades.push(generator.new_trade(
                                    book_order.id,
                                    in_order.id,
                                    book_order.price,
                                    in_order.quantity,
                                ));

                                book_order.quantity -= in_order.quantity;
                                in_order.quantity = 0;
                                break;
                            }
                        }
                    } else {
                        break;
                    }
                }
            }
        }

        if book_orders.is_empty() {
            self.order_btree.remove(&book_price);
        }

        if matched_trades.is_empty() {
            return None;
        }

        Some(matched_trades)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::generator::Generator;
    use crate::types::{BookSide, Order, OrderSide};

    // Helper constructor to reduce boilerplates in test setups
    fn make_order(id: u64, side: OrderSide, price: u64, quantity: u64) -> Order {
        Order {
            id,
            side,
            price,
            quantity,
        }
    }

    #[test]
    fn test_add_order_and_price_level_count() {
        let mut book = HalfBook::new(BookSide::Asks);
        assert_eq!(book.len(), 0);

        book.add_order(make_order(1, OrderSide::Sell, 100, 10));
        book.add_order(make_order(2, OrderSide::Sell, 100, 5));
        assert_eq!(
            book.len(),
            1,
            "Multiple orders at same price must share 1 BTreeMap level"
        );

        book.add_order(make_order(3, OrderSide::Sell, 105, 10));
        assert_eq!(book.len(), 2, "Distinct prices must create new levels");
    }

    #[test]
    fn test_exact_single_order_match_and_level_cleanup() {
        let mut book = HalfBook::new(BookSide::Asks);
        let mut generator = Generator::new();

        book.add_order(make_order(1, OrderSide::Sell, 100, 10));
        let mut taker = make_order(2, OrderSide::Buy, 100, 10);

        let trades = book.match_and_reduce(&mut generator, &mut taker);

        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].maker_oid, 1);
        assert_eq!(trades[0].taker_oid, 2);
        assert_eq!(trades[0].price, 100);
        assert_eq!(trades[0].quantity, 10);

        assert_eq!(taker.quantity, 0, "Taker should be fully filled");
        assert_eq!(
            book.len(),
            0,
            "Empty price level entry must be removed from BTreeMap"
        );
    }

    #[test]
    fn test_partial_fill_taker_larger_sweeps_and_drains_level() {
        let mut book = HalfBook::new(BookSide::Asks);
        let mut generator = Generator::new();

        book.add_order(make_order(1, OrderSide::Sell, 100, 10));
        let mut taker = make_order(2, OrderSide::Buy, 100, 25);

        let trades = book.match_and_reduce(&mut generator, &mut taker);

        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].quantity, 10);
        assert_eq!(taker.quantity, 15, "Taker should have 15 units remaining");
        assert_eq!(book.len(), 0, "Fully drained level should be removed");
    }

    #[test]
    fn test_partial_fill_maker_larger_remains_in_book() {
        let mut book = HalfBook::new(BookSide::Asks);
        let mut generator = Generator::new();

        book.add_order(make_order(1, OrderSide::Sell, 100, 20));
        let mut taker = make_order(2, OrderSide::Buy, 100, 5);

        let trades = book.match_and_reduce(&mut generator, &mut taker);

        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].quantity, 5);
        assert_eq!(taker.quantity, 0);
        assert_eq!(
            book.len(),
            1,
            "Price level must remain in book with leftover 15 units"
        );
    }

    #[test]
    fn test_fifo_time_priority_matching() {
        let mut book = HalfBook::new(BookSide::Asks);
        let mut generator = Generator::new();

        // Two maker orders queued at the same price
        book.add_order(make_order(10, OrderSide::Sell, 100, 10));
        book.add_order(make_order(11, OrderSide::Sell, 100, 10));

        let mut taker = make_order(20, OrderSide::Buy, 100, 15);

        let trades = book.match_and_reduce(&mut generator, &mut taker);

        assert_eq!(trades.len(), 2);
        assert_eq!(
            trades[0].maker_oid, 10,
            "Earlier order (id: 10) must match first"
        );
        assert_eq!(trades[0].quantity, 10);

        assert_eq!(
            trades[1].maker_oid, 11,
            "Later order (id: 11) matches second"
        );
        assert_eq!(trades[1].quantity, 5);

        assert_eq!(taker.quantity, 0);
        assert_eq!(book.len(), 1, "Order 11 should remain with 5 units");
    }

    #[test]
    fn test_buy_taker_sweeps_multiple_ask_price_levels() {
        let mut book = HalfBook::new(BookSide::Asks);
        let mut generator = Generator::new();

        book.add_order(make_order(1, OrderSide::Sell, 100, 10));
        book.add_order(make_order(2, OrderSide::Sell, 101, 10));
        book.add_order(make_order(3, OrderSide::Sell, 102, 10));

        // Buy limit is 101, so it shouldn't match against Ask at 102
        let mut taker = make_order(99, OrderSide::Buy, 101, 25);

        let trades = book.match_and_reduce(&mut generator, &mut taker);

        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].price, 100);
        assert_eq!(trades[0].quantity, 10);

        assert_eq!(trades[1].price, 101);
        assert_eq!(trades[1].quantity, 10);

        assert_eq!(
            taker.quantity, 5,
            "5 units remaining because 102 is above limit 101"
        );
        assert_eq!(book.len(), 1, "Level 102 should remain untouched");
    }

    #[test]
    fn test_sell_taker_sweeps_multiple_bid_price_levels() {
        let mut book = HalfBook::new(BookSide::Bids);
        let mut generator = Generator::new();

        // Bids at 105, 104, 103
        book.add_order(make_order(1, OrderSide::Buy, 105, 10));
        book.add_order(make_order(2, OrderSide::Buy, 104, 10));
        book.add_order(make_order(3, OrderSide::Buy, 103, 10));

        // Sell limit is 104, so it shouldn't match against Bid at 103
        let mut taker = make_order(99, OrderSide::Sell, 104, 25);

        let trades = book.match_and_reduce(&mut generator, &mut taker);

        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].price, 105, "Must match best Bid (105) first");
        assert_eq!(trades[0].quantity, 10);

        assert_eq!(trades[1].price, 104, "Matches second best Bid (104)");
        assert_eq!(trades[1].quantity, 10);

        assert_eq!(
            taker.quantity, 5,
            "5 units remaining because 103 is below limit 104"
        );
        assert_eq!(book.len(), 1, "Level 103 should remain untouched");
    }

    #[test]
    fn test_non_crossing_prices_produce_no_trades() {
        let mut book = HalfBook::new(BookSide::Asks);
        let mut generator = Generator::new();

        book.add_order(make_order(1, OrderSide::Sell, 105, 10));
        let mut taker = make_order(2, OrderSide::Buy, 100, 10); // Buy limit 100 < Ask 105

        let trades = book.match_and_reduce(&mut generator, &mut taker);

        assert!(
            trades.is_empty(),
            "No trade should occur when prices don't cross"
        );
        assert_eq!(taker.quantity, 10);
        assert_eq!(book.len(), 1);
    }

    #[test]
    fn test_zero_quantity_taker_returns_early() {
        let mut book = HalfBook::new(BookSide::Asks);
        let mut generator = Generator::new();

        book.add_order(make_order(1, OrderSide::Sell, 100, 10));
        let mut taker = make_order(2, OrderSide::Buy, 100, 0); // 0 quantity

        let trades = book.match_and_reduce(&mut generator, &mut taker);

        assert!(trades.is_empty());
        assert_eq!(book.len(), 1);
    }
}

#[cfg(test)]
mod remove_order_tests {
    use super::*;
    use crate::types::{BookSide, Order, OrderSide};

    fn make_order(id: u64, side: OrderSide, price: u64, quantity: u64) -> Order {
        Order {
            id,
            side,
            price,
            quantity,
        }
    }

    #[test]
    fn test_remove_order_from_middle_of_queue() {
        let mut book = HalfBook::new(BookSide::Asks);

        // Add 3 orders at price level 100
        book.add_order(make_order(1, OrderSide::Sell, 100, 10));
        book.add_order(make_order(2, OrderSide::Sell, 100, 20));
        book.add_order(make_order(3, OrderSide::Sell, 100, 30));

        assert_eq!(book.len(), 1);

        // Remove order 2 (middle of VecDeque)
        let removed = book.remove_order(2);
        assert!(removed, "Expected remove_order to return true for existing order");

        // Verify level remains because orders 1 and 3 are still present
        assert_eq!(book.len(), 1, "Price level must remain active while orders remain");

        // Verify order 2 cannot be removed a second time
        assert!(!book.remove_order(2), "Double cancellation must return false");
    }

    #[test]
    fn test_remove_order_head_and_tail() {
        let mut book = HalfBook::new(BookSide::Asks);

        book.add_order(make_order(10, OrderSide::Sell, 100, 10));
        book.add_order(make_order(11, OrderSide::Sell, 100, 20));
        book.add_order(make_order(12, OrderSide::Sell, 100, 30));

        // Remove head (front)
        assert!(book.remove_order(10));

        // Remove tail (back)
        assert!(book.remove_order(12));

        assert_eq!(book.len(), 1, "Price level should still hold order 11");
    }

    #[test]
    fn test_remove_last_order_cleans_up_btree_level() {
        let mut book = HalfBook::new(BookSide::Asks);

        book.add_order(make_order(1, OrderSide::Sell, 100, 10));
        book.add_order(make_order(2, OrderSide::Sell, 105, 20));

        assert_eq!(book.len(), 2);

        // Remove only order at level 100
        assert!(book.remove_order(1));

        assert_eq!(
            book.len(),
            1,
            "Level 100 must be removed from BTreeMap when empty"
        );

        // Remove only order at level 105
        assert!(book.remove_order(2));

        assert_eq!(
            book.len(),
            0,
            "BTreeMap should be completely empty after all orders are removed"
        );
    }

    #[test]
    fn test_remove_non_existent_order() {
        let mut book = HalfBook::new(BookSide::Asks);

        book.add_order(make_order(1, OrderSide::Sell, 100, 10));

        // Attempt to remove an ID that was never added
        assert!(!book.remove_order(999));
        assert_eq!(book.len(), 1, "Book state should remain unchanged");
    }

    #[test]
    fn test_matching_after_order_removal() {
        let mut book = HalfBook::new(BookSide::Asks);
        let mut generator = Generator::new();

        book.add_order(make_order(1, OrderSide::Sell, 100, 10));
        book.add_order(make_order(2, OrderSide::Sell, 100, 20));

        // Cancel order 1 (front of queue)
        assert!(book.remove_order(1));

        // Incoming taker should now match directly against order 2
        let mut taker = make_order(3, OrderSide::Buy, 100, 15);
        let trades = book.match_and_reduce(&mut generator, &mut taker);

        assert_eq!(trades.len(), 1);
        assert_eq!(trades[0].maker_oid, 2, "Trade must execute against remaining order 2");
        assert_eq!(trades[0].quantity, 15);
        assert_eq!(taker.quantity, 0);
    }
}
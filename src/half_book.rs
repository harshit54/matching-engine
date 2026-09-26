use crate::generator::Generator;
use crate::types::{Order, Side, Trade};
use std::cmp::Ordering;
use std::collections::btree_map::OccupiedEntry;
use std::collections::{BTreeMap, VecDeque};
use std::fmt;

pub struct HalfBook {
    orders: BTreeMap<u64, VecDeque<Order>>,
}

impl fmt::Display for HalfBook {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for level in self.orders.iter() {
            write!(f, "\n\t{}: {:?}", level.0, level.1)?;
        }

        write!(f, "\n")
    }
}

impl HalfBook {
    pub fn len(&self) -> usize {
        self.orders.len()
    }

    pub fn new() -> Self {
        Self {
            orders: BTreeMap::new(),
        }
    }

    pub fn add_order(&mut self, order: Order) {
        self.orders.entry(order.price).or_default().push_back(order)
    }

    pub fn get_best_price_level(
        &mut self,
        side: &Side,
    ) -> Option<OccupiedEntry<'_, u64, VecDeque<Order>>> {
        // BTreeMap first_entry is the minimum entry.
        match side {
            Side::Buy => self.orders.last_entry(),
            Side::Sell => self.orders.first_entry(),
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
        let price_level = self.get_best_price_level(&in_order.side)?;

        let book_price = *price_level.key();
        let book_orders = price_level.into_mut();

        let mut matched_trades: Vec<Trade> = Vec::new();

        loop {
            match book_orders.front_mut() {
                None => break,

                Some(book_order) => {
                    match book_price.cmp(&in_order.price) {
                        // no match
                        Ordering::Greater => break,

                        Ordering::Equal | Ordering::Less => {
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
                        }
                    }
                }
            }
        }

        if book_orders.is_empty() {
            self.orders.remove(&book_price);
        }

        if matched_trades.is_empty() {
            return None;
        }

        Some(matched_trades)
    }
}

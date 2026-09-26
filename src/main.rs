/*
Matching Engine
Structs:
Order[ID, Price, Qty]
Trade[OrderID, Price, Qty]

Functions:
1. Add Order -> Price, Qty -> ID, []MatchedTrades --> Only Limit Orders For Now
2. Remove Order -> ID -> Success/Fail
*/
use order_book::OrderBook;

mod generator;
mod half_book;
mod order_book;

#[derive(Debug)]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug)]
struct Order {
    id: u64,
    side: Side,
    price: u64,
    quantity: u64,
}

#[derive(Debug)]
struct Trade {
    id: u64,
    maker_oid: u64,
    taker_oid: u64,
    price: u64,
    quantity: u64,
}

fn main() {
    let mut ob = OrderBook::new();

    ob.add_order(Side::Sell, 1, 1);
    ob.add_order(Side::Buy, 1, 1);

    println!("{}", ob);
}

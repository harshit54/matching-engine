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
use types::Side;

mod generator;
mod half_book;
mod order_book;
mod types;

fn main() {
    let mut ob = OrderBook::new();

    ob.add_order(Side::Sell, 1, 1);
    ob.add_order(Side::Buy, 1, 1);

    println!("{}", ob);
}

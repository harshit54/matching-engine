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
use types::OrderSide;

mod generator;
mod half_book;
mod order_book;
mod types;

fn main() {
    let mut ob = OrderBook::new();

    let o1 = ob.add_order(OrderSide::Buy, 105, 10);
    let o2 = ob.add_order(OrderSide::Buy, 105, 10);

    println!("{}", ob);
}

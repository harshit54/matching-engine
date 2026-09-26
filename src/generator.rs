use crate::{Order, Side, Trade};

#[derive(Debug)]
pub struct Generator {
    oid: u64,
    tid: u64,
}

impl Generator {
    pub fn new() -> Generator {
        return Generator { oid: 0, tid: 0 };
    }

    pub fn new_order(&mut self, side: Side, price: u64, quantity: u64) -> Order {
        self.oid += 1;
        Order {
            id: self.oid,
            side,
            price,
            quantity,
        }
    }

    pub fn new_trade(&mut self, maker_oid: u64, taker_oid: u64, price: u64, quantity: u64) -> Trade {
        self.tid += 1;
        Trade {
            id: self.tid,
            maker_oid,
            taker_oid,
            price,
            quantity,
        }
    }
}
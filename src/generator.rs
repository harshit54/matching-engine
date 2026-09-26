use crate::types::{Order, OrderSide, Trade};

#[derive(Debug)]
pub struct Generator {
    oid: u64,
    tid: u64,
}

impl Generator {
    pub fn new() -> Generator {
        return Generator { oid: 0, tid: 0 };
    }

    pub fn new_order(&mut self, side: OrderSide, price: u64, quantity: u64) -> Order {
        self.oid += 1;
        Order {
            id: self.oid,
            side,
            price,
            quantity,
        }
    }

    pub fn new_trade(
        &mut self,
        maker_oid: u64,
        taker_oid: u64,
        price: u64,
        quantity: u64,
    ) -> Trade {
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

#[cfg(test)]
mod tests {
    use crate::generator::Generator;
    use crate::types::OrderSide::{Buy, Sell};

    #[test]
    fn test_generator_field_assignment_and_initial_ids() {
        let mut generator = Generator::new();

        let o1 = generator.new_order(Buy, 10, 100);
        assert_eq!(o1.id, 1, "First order ID must start at 1");
        assert_eq!(o1.side, Buy);
        assert_eq!(o1.price, 10);
        assert_eq!(o1.quantity, 100);

        let t1 = generator.new_trade(10, 20, 30, 40);
        assert_eq!(t1.id, 1, "First trade ID must start at 1");
        assert_eq!(t1.maker_oid, 10);
        assert_eq!(t1.taker_oid, 20);
        assert_eq!(t1.price, 30);
        assert_eq!(t1.quantity, 40);
    }

    #[test]
    fn test_generator_sequential_increments() {
        let mut generator = Generator::new();

        // Generate orders in sequence
        let o1 = generator.new_order(Buy, 100, 10);
        let o2 = generator.new_order(Sell, 105, 5);
        let o3 = generator.new_order(Buy, 99, 20);

        assert_eq!(o1.id, 1);
        assert_eq!(o2.id, 2);
        assert_eq!(o3.id, 3);

        // Generate trades in sequence
        let t1 = generator.new_trade(o1.id, o2.id, 100, 5);
        let t2 = generator.new_trade(o3.id, o2.id, 99, 10);

        assert_eq!(t1.id, 1);
        assert_eq!(t2.id, 2);
    }

    #[test]
    fn test_interleaved_id_isolation() {
        let mut generator = Generator::new();

        // Interleave order and trade creation to prove `oid` and `tid` counters are isolated
        let o1 = generator.new_order(Buy, 100, 10); // oid: 1
        let t1 = generator.new_trade(1, 2, 100, 5); // tid: 1
        let t2 = generator.new_trade(1, 3, 100, 5); // tid: 2
        let o2 = generator.new_order(Sell, 105, 5); // oid: 2

        assert_eq!(o1.id, 1);
        assert_eq!(o2.id, 2);
        assert_eq!(t1.id, 1);
        assert_eq!(t2.id, 2);
    }

    #[test]
    fn test_zero_quantity_and_price_edge_cases() {
        let mut generator = Generator::new();

        // Engine market/cancel constructs often pass 0 price or qty through generator
        let o1 = generator.new_order(Buy, 0, 0);
        assert_eq!(o1.id, 1);
        assert_eq!(o1.price, 0);
        assert_eq!(o1.quantity, 0);
    }
}

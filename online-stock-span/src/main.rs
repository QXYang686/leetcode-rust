struct StockSpanner {
    index_stack: Vec<i32>,
    price_stack: Vec<i32>,
    index: i32,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {

    fn new() -> Self {
        StockSpanner {
            index_stack: vec![-1],
            price_stack: vec![i32::MAX],
            index: -1,
        }
    }

    fn next(&mut self, price: i32) -> i32 {
        self.index += 1;
        while *self.price_stack.last().unwrap() <= price {
            self.price_stack.pop();
            self.index_stack.pop();
        }
        let result = self.index - self.index_stack.last().unwrap();
        self.index_stack.push(self.index);
        self.price_stack.push(price);
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::StockSpanner;

    #[test]
    fn example1() {
        let mut spanner = StockSpanner::new();
        assert_eq!(1, spanner.next(100));
        assert_eq!(1, spanner.next(80));
        assert_eq!(1, spanner.next(60));
        assert_eq!(2, spanner.next(70));
        assert_eq!(1, spanner.next(60));
        assert_eq!(4, spanner.next(75));
        assert_eq!(6, spanner.next(85));
    }
}

/**
 * Your StockSpanner object will be instantiated and called as such:
 * let obj = StockSpanner::new();
 * let ret_1: i32 = obj.next(price);
 */
fn main() {}
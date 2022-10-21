struct StockSpanner {
    data: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {

    fn new() -> Self {
        StockSpanner {
            data: vec![],
        }
    }

    fn next(&mut self, price: i32) -> i32 {
        self.data.push(price);
        self.data.iter().rev().take_while(|&x| *x <= price).count() as i32
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
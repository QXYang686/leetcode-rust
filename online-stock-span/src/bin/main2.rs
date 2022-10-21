struct StockSpanner {
    prices: Vec<i32>,
    results: Vec<i32>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl StockSpanner {

    fn new() -> Self {
        StockSpanner {
            prices: vec![],
            results: vec![],
        }
    }

    fn next(&mut self, price: i32) -> i32 {
        self.prices.push(price);
        self.results.push(1);
        let mut pos = self.results.len() as i32 - 2;
        while pos >= 0 && self.prices[pos as usize] <= price {
            *self.results.last_mut().unwrap() += self.results[pos as usize];
            pos -= self.results[pos as usize];
        }
        *self.results.last().unwrap()
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
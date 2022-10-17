use std::cmp::max;
use std::collections::HashMap;

impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut result = 1;
        // 初始化篮子
        let mut buckets = HashMap::new();
        buckets.insert(fruits[0], 1);
        // 窗口滑动
        let (mut from, mut to) = (0, 0);
        loop {
            // enlarge
            while to + 1 < fruits.len() && (buckets.contains_key(&fruits[to + 1]) || buckets.len() < 2) {
                if buckets.contains_key(&fruits[to + 1]) {
                    *buckets.get_mut(&fruits[to + 1]).unwrap() += 1;
                } else if buckets.len() < 2 {
                    buckets.insert(fruits[to + 1], 1);
                }
                to += 1;
            }
            // update record
            result = max(result, to - from + 1);
            // shrink
            while from <= to && buckets.len() >= 2 {
                *buckets.get_mut(&fruits[from]).unwrap() -= 1;
                if *buckets.get(&fruits[from]).unwrap() == 0 {
                    buckets.remove(&fruits[from]);
                }
                from += 1;
            }
            // slided to end, return
            if to + 1 >= fruits.len() {
                return result as i32;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(3, Solution::total_fruit(vec![1, 2, 1]))
    }

    #[test]
    fn example2() {
        assert_eq!(3, Solution::total_fruit(vec![0, 1, 2, 2]))
    }

    #[test]
    fn example3() {
        assert_eq!(4, Solution::total_fruit(vec![1, 2, 3, 2, 2]))
    }

    #[test]
    fn example4() {
        assert_eq!(5, Solution::total_fruit(vec![3, 3, 3, 1, 2, 1, 1, 2, 3, 3, 4]))
    }
}

fn main() {}

struct Solution;
use std::cmp::max;
use std::collections::HashSet;

// Brute-force, won't work.
impl Solution {
    pub fn total_fruit(fruits: Vec<i32>) -> i32 {
        let mut result = 0;
        for from in 0..=fruits.len() {
            for to in from + result..=fruits.len() {
                if fruits[from..to].iter().collect::<HashSet<_>>().len() <= 2 {
                    // println!("{} {}", from, to);
                    result = max(result, to - from);
                }
            }
        }
        result as i32
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

fn main() {
    println!("Hello, world!");
}

struct Solution;
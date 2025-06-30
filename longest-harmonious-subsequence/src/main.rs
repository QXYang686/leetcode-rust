use std::collections::HashMap;

impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut ans = 0;
        let mut count = HashMap::new();
        for num in nums {
            *count.entry(num).or_insert(0) += 1;
        }
        for (key, value) in count.iter() {
            if let Some(next) = count.get(&(key + 1)) {
                ans = ans.max(value + next);
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(5, Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]))
    }

    #[test]
    fn example2() {
        assert_eq!(2, Solution::find_lhs(vec![1, 2, 3, 4]))
    }

    #[test]
    fn example3() {
        assert_eq!(0, Solution::find_lhs(vec![1, 1, 1, 1]))
    }
}

fn main() {
    println!("Hello, world!");
}
struct Solution;

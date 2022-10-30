// out of time
use std::usize;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = usize::MAX / 2;
        for i in 0..=nums.len() {
            for j in i..=(i + ans).min(nums.len()) {
                if nums[i..j].iter().sum::<i32>() >= k {
                    ans = ans.min(j - i);
                    break;
                }
            }
        }
        ans as i32
    }
}

mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(1, Solution::shortest_subarray(vec![1], 1));
    }

    #[test]
    fn example2() {
        assert_eq!(-1, Solution::shortest_subarray(vec![1,2], 4));
    }

    #[test]
    fn example3() {
        assert_eq!(3, Solution::shortest_subarray(vec![2, -1,2], 3));
    }
}

fn main() {
    println!("Hello, world!");
}

struct Solution;
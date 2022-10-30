use std::collections::VecDeque;

impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut pre_sums = vec![0i64; nums.len() + 1];
        for i in 1..pre_sums.len() {
            pre_sums[i] = pre_sums[i - 1] + nums[i - 1] as i64;
        }
        // println!("nums:{nums:?},pre_sums:{pre_sums:?}");
        let mut ans = pre_sums.len();
        let mut deque = VecDeque::new();
        for (i, &cur_sum) in pre_sums.iter().enumerate() {
            while !deque.is_empty() && cur_sum - pre_sums[deque[0]] >= k as i64 {
                ans = ans.min(i - deque.pop_front().unwrap());
            }
            while !deque.is_empty() && pre_sums[*deque.iter().last().unwrap()] >= cur_sum {
                deque.pop_back();
            }
            deque.push_back(i);
        }
        if ans > nums.len() { -1 } else { ans as i32 }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(1, Solution::shortest_subarray(vec![1], 1));
    }

    #[test]
    fn example2() {
        assert_eq!(-1, Solution::shortest_subarray(vec![1, 2], 4));
    }

    #[test]
    fn example3() {
        assert_eq!(3, Solution::shortest_subarray(vec![2, -1, 2], 3));
    }

    #[test]
    fn examplex() {
        let nums = vec![-100_000i32;100_000];
        println!("{}", nums.iter().fold(0i64, |sum, &value| sum + value as i64));
        println!("{}", i32::MIN);
        println!("{}", i32::MAX);
        assert_eq!(-1, Solution::shortest_subarray(nums, 1000_000_000));
    }
}

fn main() {
    println!("Hello, world!");
}

struct Solution;
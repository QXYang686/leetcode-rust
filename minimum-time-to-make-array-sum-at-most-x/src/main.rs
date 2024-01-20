use std::cmp::max;

impl Solution {
    /// 在nums2升序的情况下
    /// dp[j][i] 表示前j个元素操作i次能减少的最大总值
    /// dp[j][i] = max(dp[j-1][i-1]+i*nums2[j-1]+nums1[j-1], dp[j-1][i])
    /// 空间压缩后
    /// dp[i] = max(dp[i], dp[i-1]+i*nums2[i-1]+nums1[i-1])
    pub fn minimum_time(nums1: Vec<i32>, nums2: Vec<i32>, x: i32) -> i32 {
        let s1: i32 = nums1.iter().sum();
        let s2: i32 = nums2.iter().sum();
        let n = nums1.len();
        let mut dp = vec![0; n + 1];
        let mut nums: Vec<_> = nums1.into_iter().zip(nums2.into_iter()).collect();
        nums.sort_unstable_by(|a,b| a.1.cmp(&b.1));
        for j in 1..=n {
            for i in (1..=j).rev() {
                dp[i] = max(dp[i], dp[i - 1] + i as i32 * nums[j - 1].1 + nums[j - 1].0);
            }
        }
        for i in 0..=n {
            if s2 * i as i32 + s1 - dp[i] <= x {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(3, Solution::minimum_time(vec![1,2,3], vec![1,2,3], 4));
    }
    #[test]
    fn example2() {
        assert_eq!(-1, Solution::minimum_time(vec![1,2,3], vec![3,3,3], 4));
    }
}

fn main() {
    println!("Hello, world!");
}
struct Solution;

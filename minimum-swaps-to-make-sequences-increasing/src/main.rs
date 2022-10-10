use std::cmp::{max, min};

impl Solution {
    pub fn min_swap(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let (mut keep, mut swap) = (0, 0);
        let (mut pre1, mut pre2) = (-1, -1);
        for (&num1, &num2) in nums1.iter().zip(nums2.iter()) {
            let (last_keep, last_swap) = (keep, swap);
            match (max(pre1, pre2) < min(num1, num2), num1 > pre1 && num2 > pre2) {
                // all way is available, choose the best one
                (true, _) => {
                    keep = min(last_keep, last_swap);
                    swap = min(last_keep, last_swap) + 1;
                }
                // keep->keep, swap->swap
                (false, true) => {
                    keep = last_keep;
                    swap = last_swap + 1;
                }
                // keep->swap, swap->keep
                (false, false) => {
                    keep = last_swap;
                    swap = last_keep + 1;
                }
            };
            pre1 = num1;
            pre2 = num2;
        }
        min(keep, swap)
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(1, Solution::min_swap(vec![1, 3, 5, 4], vec![1, 2, 3, 7]));
    }

    #[test]
    fn example2() {
        assert_eq!(1, Solution::min_swap(vec![0, 3, 5, 8, 9], vec![2, 1, 4, 6, 9]));
    }
}

struct Solution;

fn main() {}
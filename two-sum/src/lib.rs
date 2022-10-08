impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, &num1) in nums.iter().enumerate() {
            for (j, &num2) in nums.iter().skip(i + 1).enumerate() {
                if num1 + num2 == target {
                    return vec![i as i32, (i + j + 1) as i32];
                }
            }
        }
        panic!();
    }
}

struct Solution;
#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test_example_1() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![2,7,11,15], 9));
    }

    #[test]
    fn test_example_2() {
        assert_eq!(vec![1, 2], Solution::two_sum(vec![3,2,4], 6));
    }

    #[test]
    fn test_example_3() {
        assert_eq!(vec![0, 1], Solution::two_sum(vec![3,3], 6));
    }
}
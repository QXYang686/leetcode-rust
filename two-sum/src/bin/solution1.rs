use std::collections::HashMap;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hashmap = HashMap::new();
        for (index, &value) in nums.iter().enumerate() {
            if hashmap.contains_key(&value) {
                return vec![*hashmap.get(&value).unwrap(), index as i32];
            }
            hashmap.insert(target - value, index as i32);
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

fn main() {

}
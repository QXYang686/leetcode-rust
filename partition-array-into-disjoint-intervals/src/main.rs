impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let (mut pos, mut max_value, mut cur_max_value) = (0usize, i32::MIN, i32::MAX);
        for (cur_pos, &num) in nums.iter().enumerate() {
            max_value = max_value.max(num);
            if cur_max_value > num {
                pos = cur_pos;
                cur_max_value = max_value;
            }
        }
        (pos + 1) as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(3, Solution::partition_disjoint(vec![5,0,3,8,6]));
    }

    #[test]
    fn example2() {
        assert_eq!(4, Solution::partition_disjoint(vec![1,1,1,0,6,12]));
    }
}

fn main() {
    println!("Hello, world!");
}
struct Solution;
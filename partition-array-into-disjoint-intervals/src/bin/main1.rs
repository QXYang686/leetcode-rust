impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let (mut min, mut max) = (i32::MAX, i32::MIN);
        let max_nums: Vec<_> = nums.iter().map(|x| {
            max = max.max(*x);
            max
        }).collect();
        let min_nums: Vec<_> = nums.iter().rev().map(|x| {
            min = min.min(*x);
            min
        }).collect();
        println!("max_nums:{max_nums:?},min_nums:{min_nums:?}");
        for (index, (max, min)) in max_nums.iter().zip(min_nums.iter().rev().skip(1)).enumerate() {
            if max <= min {
                return (index + 1) as i32;
            }
        }
        panic!();
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
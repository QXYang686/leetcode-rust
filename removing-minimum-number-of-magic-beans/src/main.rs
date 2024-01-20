use std::cmp::max;

impl Solution {
    pub fn minimum_removal(mut beans: Vec<i32>) -> i64 {
        beans.sort_unstable();
        let sum: i64 = beans.iter().map(|&x| x as i64).sum();
        let mut max_left = 0;
        for (i, &num) in beans.iter().enumerate() {
            max_left = max((beans.len() -i) as i64 * num as i64, max_left);
        }
        sum - max_left
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(4, Solution::minimum_removal(vec![4,1,6,5]))
    }

    #[test]
    fn example2() {
        assert_eq!(7, Solution::minimum_removal(vec![2,10,3,2]))
    }
}

fn main() {
    println!("Hello, world!");
}
struct Solution;
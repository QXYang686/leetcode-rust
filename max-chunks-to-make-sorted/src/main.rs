use std::cmp::max;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let mut chunk_end = 0;
        arr.iter().enumerate()
            .filter(|x| {
                chunk_end = max(chunk_end, *x.1);
                x.0 as i32 >= chunk_end
            }).count() as i32
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(1, Solution::max_chunks_to_sorted(vec![4,3,2,1,0]));
    }
    #[test]
    fn example2() {
        assert_eq!(4, Solution::max_chunks_to_sorted(vec![1,0,2,3,4]));
    }
}

fn main() {
    println!("Hello, world!");
}
struct Solution;
impl Solution {
    pub fn split_array(nums: Vec<i32>, k: i32) -> i32 {
        let (mut low, mut high) = (*nums.iter().max().unwrap(), nums.iter().sum::<i32>());
        while low < high {
            let mid = low + (high - low) / 2;
            let mut seg_sum = 0;
            let mut cnt = 1;
            for &num in nums.iter() {
                if seg_sum + num > mid {
                    seg_sum = num;
                    cnt += 1;
                } else {
                    seg_sum += num;
                }
            }
            if cnt > k {
                low = mid + 1;
            } else {
                high = mid;
            }
        }
        low
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(18, Solution::split_array(vec![7,2,5,10,8], 2));
    }

    #[test]
    fn example2() {
        assert_eq!(9, Solution::split_array(vec![1,2,3,4,5], 2));
    }

    #[test]
    fn example3() {
        assert_eq!(4, Solution::split_array(vec![1,4,4], 3));
    }
}

fn main() {
    println!("Hello, world!");
}
struct Solution;
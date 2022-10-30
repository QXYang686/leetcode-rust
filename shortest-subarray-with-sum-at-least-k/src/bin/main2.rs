// out of time
// 通过O(n)的时间完成前缀和构造后，分段求和的时间复杂度为O(1)
impl Solution {
    pub fn shortest_subarray(nums: Vec<i32>, k: i32) -> i32 {
        let mut pre_sums = vec![0;nums.len()+1];
        for i in 1..pre_sums.len() {
            pre_sums[i] = pre_sums[i-1] + nums[i-1];
        }
        // println!("nums:{nums:?},pre_sums:{pre_sums:?}");
        let mut ans = usize::MAX / 2;
        for i in 0..nums.len() {
            for j in (i+1)..pre_sums.len().min(i+ans) {
                if pre_sums[j] - pre_sums[i] >= k {
                    ans = ans.min(j - i);
                    break;
                }
            }
        }
        ans as i32
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
        assert_eq!(-1, Solution::shortest_subarray(vec![1,2], 4));
    }

    #[test]
    fn example3() {
        assert_eq!(3, Solution::shortest_subarray(vec![2, -1,2], 3));
    }
}

fn main() {
    println!("Hello, world!");
}

struct Solution;
impl Solution {
    pub fn alternating_subarray(nums: Vec<i32>) -> i32 {
        // let mut ans = -1;
        // for i in 0..nums.len() {
        //     for j in i + 1..nums.len() {
        //         let length = j - i + 1;
        //         if nums[j] - nums[j - 1] == (length as i32 - 1) % 2 {
        //             ans = ans.max(length as i32);
        //         }
        //     }
        // }
        // ans
        let mut ans = -1;
        let mut from = 0;
        for i in 1..nums.len() {
            let length = i - from + 1;
            if nums[i] - nums[from] == (length - 1) as i32 % 2 {
                // 满足条件 继续推进
                ans = ans.max(length as i32);
            } else {
                // 不满足条件
                if nums[i] - nums[i - 1] == 1 {
                    // 但前面两个是可用的 那么跳转到前面两个
                    from = i - 1;
                    ans = ans.max(2);
                } else {
                    // 不可用 下一轮直接从当前位置开始算起
                    from = i;
                }
            }
        }
        ans
    }
}

mod tests {
    use crate::Solution;

    #[test]
    fn example_1() {
        assert_eq!(4, Solution::alternating_subarray(vec![2,3,4,3,4]));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::alternating_subarray(vec![4,5,6]));
    }
}

fn main() {
    println!("Hello, world!");
}
struct Solution;

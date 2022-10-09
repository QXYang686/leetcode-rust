use std::cmp::max;

impl Solution {
    // ()(())
    // => 0 + ()(())
    // => 0 + 0 + ) + (())
    // => 0+1 + (())
    // => 1 + 0 + ())
    // => 1 + 0 + 0 + ))
    // => 1 + 0+1 + )
    // => 1+1*2
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut stack = vec![0];
        for x in s.chars() {
            if x == '(' { // 遇到左括号，需要计算该左括号内部的分数，初始状态下内部为空串，分数为0
                stack.push(0);
            } else { // 遇到右括号，内部结果已计算完成(栈顶)。结果为0，空串->1；结果不为0，(A)->2*A。结果追加到栈顶元素。
                let value = stack.pop().unwrap();
                let top = stack.pop().unwrap();
                stack.push(top + max(1, 2 * value));
            }
        }
        stack[0]
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example_1() {
        assert_eq!(1, Solution::score_of_parentheses(String::from("()")));
    }

    #[test]
    fn example_2() {
        assert_eq!(2, Solution::score_of_parentheses(String::from("(())")));
    }

    #[test]
    fn example_3() {
        assert_eq!(2, Solution::score_of_parentheses(String::from("()()")));
    }

    #[test]
    fn example_4() {
        assert_eq!(6, Solution::score_of_parentheses(String::from("(()(()))")));
    }
}

struct Solution;
fn main() {

}
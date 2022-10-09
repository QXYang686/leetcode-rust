impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        Self::divide(&s)
    }

    // 分治法
    // () -> 1
    // ()()() -> () + ()()
    // (...) -> 2 * ...
    fn divide(s: &str) -> i32 {
        if s.len() == 2 {1}
        else {
            let mut bal = 0;
            for (index, value) in s.chars().enumerate() {
                bal += if value == '(' {1} else {-1};
                if bal == 0 {
                    return if index == s.len() - 1 {
                        2 * Self::divide(&s[1..index])
                    } else {
                        Self::divide(&s[..index + 1]) + Self::divide(&s[index + 1..])
                    }
                }
            }
            panic!();
        }
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
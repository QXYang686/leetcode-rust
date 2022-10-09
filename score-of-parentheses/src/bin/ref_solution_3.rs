impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        let mut bal = 1;
        let mut ret = 0;
        for (x, x_pre) in s.chars().skip(1).zip(s.chars()) {
            bal += if x == '(' {1} else {-1};
            if x_pre == '(' && x == ')' {
                ret += 1 << bal;
            }
        }
        ret
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
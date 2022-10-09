impl Solution {
    pub fn score_of_parentheses(s: String) -> i32 {
        enum Unit {Left, Value(i32)}
        let mut stack = Vec::new();
        for x in s.chars() {
            match x {
                '(' => stack.push(Unit::Left),
                ')' => {
                    let mut value = 0;
                    loop {
                        match stack.pop().unwrap() {
                            Unit::Left => {
                                stack.push(Unit::Value(if value == 0 {1} else {2 * value}));
                                break;
                            },
                            Unit::Value(v) => {
                                value += v;
                            },
                        }
                    }
                },
                _ => panic!(),
            }
        }
        stack.iter().map(|x| {
            match *x {
                Unit::Value(x) => x,
                _ => panic!(),
            }
        }).sum()
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
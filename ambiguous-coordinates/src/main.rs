impl Solution {
    pub fn ambiguous_coordinates(s: String) -> Vec<String> {
        let chars = &s[1..s.len()-1];
        let mut ans = vec![];
        for i in 1..chars.len() {
            let (left, right) = (Self::interpret(&chars[..i]), Self::interpret(&chars[i..]));
            for left in &left {
                for right in &right {
                    ans.push(format!("({left}, {right})"));
                }
            }
        }
        ans
    }
    fn interpret(s: &str) -> Vec<String> {
        let mut ans = vec![];
        for i in 1..s.len() {
            if s[..i].starts_with("0") && s[..i].len() > 1 {
                continue;
            } else if s[i..].ends_with("0") {
                continue;
            } else {
                ans.push(format!("{}.{}", &s[..i], &s[i..]));
            }
        }
        if s.len() == 1 || !s.starts_with("0") {
            ans.push(String::from(s));
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(vec!["(1, 23)", "(12, 3)", "(1.2, 3)", "(1, 2.3)"].into_iter().map(|x| String::from(x)).collect::<HashSet<_>>(),
                   Solution::ambiguous_coordinates(String::from("(123)")).into_iter().collect::<HashSet<_>>());
    }
    #[test]
    fn example2() {
        assert_eq!(vec!["(0.001, 1)", "(0, 0.011)"].into_iter().map(|x| String::from(x)).collect::<HashSet<_>>(),
                   Solution::ambiguous_coordinates(String::from("(00011)")).into_iter().collect::<HashSet<_>>());
    }
    #[test]
    fn example3() {
        assert_eq!(vec!["(0, 123)", "(0, 12.3)", "(0, 1.23)", "(0.1, 23)", "(0.1, 2.3)", "(0.12, 3)"].into_iter().map(|x| String::from(x)).collect::<HashSet<_>>(),
                   Solution::ambiguous_coordinates(String::from("(0123)")).into_iter().collect::<HashSet<_>>());
    }
    #[test]
    fn example4() {
        assert_eq!(vec!["(10, 0)"].into_iter().map(|x| String::from(x)).collect::<HashSet<_>>(),
                   Solution::ambiguous_coordinates(String::from("(100)")).into_iter().collect::<HashSet<_>>());
    }
}

fn main() {
    println!("Hello, world!");
}
struct Solution;
impl Solution {
    pub fn interpret(command: String) -> String {
        let mut ans = String::from(if command.starts_with('G') {"G"} else {""});
        for (pre, cur) in command.chars().zip(command.chars().skip(1)) {
            match (pre, cur) {
                (_, 'G') => ans.push('G'),
                ('(', ')') => ans.push('o'),
                ('l', ')') => ans.push_str("al"),
                _ => {},
            };
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!("Goal", Solution::interpret(String::from("G()(al)")));
    }
    #[test]
    fn example2() {
        assert_eq!("Gooooal", Solution::interpret(String::from("G()()()()(al)")));
    }
    #[test]
    fn example3() {
        assert_eq!("alGalooG", Solution::interpret(String::from("(al)G(al)()()G")));
    }
}

fn main() {
    println!("Hello, world!");
}
struct Solution;
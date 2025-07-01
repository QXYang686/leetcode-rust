impl Solution {
    pub fn possible_string_count(word: String) -> i32 {
        let mut ans = 1;
        for (pre, cur) in word.chars().zip(word.chars().skip(1)) {
            if pre == cur {
                ans += 1;
            }
        }
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(5, Solution::possible_string_count("abbcccc".into()));
    }

    #[test]
    fn example2() {
        assert_eq!(1, Solution::possible_string_count("abcd".into()));
    }

    #[test]
    fn example3() {
        assert_eq!(4, Solution::possible_string_count("aaaa".into()));
    }
}

fn main() {
    println!("Hello, world!");
}
struct Solution;

impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let (mut iter1, mut iter2) = (word1.chars(), word2.chars());
        let mut result = String::new();
        loop {
            match (iter1.next(), iter2.next()) {
                (Some(x), Some(y)) => {
                    result.push(x);
                    result.push(y);
                },
                (Some(x), None) => {
                    result.push(x);
                },
                (None, Some(y)) => {
                    result.push(y);
                },
                (None, None) => break,
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(String::from("apbqcr"), Solution::merge_alternately(String::from("abc"), String::from("pqr")))
    }
    #[test]
    fn example2() {
        assert_eq!(String::from("apbqrs"), Solution::merge_alternately(String::from("ab"), String::from("pqrs")))
    }
    #[test]
    fn example3() {
        assert_eq!(String::from("apbqcd"), Solution::merge_alternately(String::from("abcd"), String::from("pq")))
    }
}

fn main() {
    println!("Hello, world!");
}
struct Solution;
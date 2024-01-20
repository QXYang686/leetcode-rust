use std::collections::HashSet;

impl Solution {
    pub fn maximum_number_of_string_pairs(words: Vec<String>) -> i32 {
        let word_set: HashSet<_> = words.iter().filter(|word| word[1..] != word[..1]).collect();
        words.iter().filter(|word| word_set.contains(&(word[1..].to_string() + &word[..1])))
            .count() as i32 / 2
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(2, Solution::maximum_number_of_string_pairs(
            vec!["cd","ac","dc","ca","zz"].iter().map(|x| x.to_string()).collect()));
    }

    #[test]
    fn example2() {
        assert_eq!(1, Solution::maximum_number_of_string_pairs(
            vec!["ab","ba","cc"].iter().map(|x| x.to_string()).collect()
        ));
    }

    #[test]
    fn example3() {
        assert_eq!(0, Solution::maximum_number_of_string_pairs(
            vec!["aa","ab"].iter().map(|x| x.to_string()).collect()
        ));
    }

    #[test]
    fn failed() {
        assert_eq!(0, Solution::maximum_number_of_string_pairs(
            vec!["ff","tx","qr","zw","wr","jr","zt","jk","sq","xx"].iter().map(|x| x.to_string()).collect()
        ))
    }
}

fn main() {
    println!("Hello, world!");
}
struct Solution;

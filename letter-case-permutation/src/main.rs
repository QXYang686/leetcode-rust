impl Solution {
    pub fn letter_case_permutation(s: String) -> Vec<String> {
        let mut res = vec![s.to_uppercase()];
        for (i, _ch) in s.chars().enumerate().filter(|x| !x.1.is_ascii_digit()) {
            let mut permutation = res.clone();
            for str in permutation.iter_mut() {
                unsafe { str.as_bytes_mut()[i] += 'a' as u8 - 'A' as u8; }
            }
            res.append(&mut permutation);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::Solution;

    #[test]
    fn example1() {
        let expected: HashSet<_> = vec!["a1b2", "a1B2", "A1b2", "A1B2"].iter().map(|x| x.to_string()).collect();
        let actual: HashSet<_> = Solution::letter_case_permutation(String::from("a1b2")).into_iter().collect();
        assert_eq!(expected, actual);
    }
    #[test]
    fn example2() {
        let expected: HashSet<_> = vec!["3z4","3Z4"].iter().map(|x| x.to_string()).collect();
        let actual: HashSet<_> = Solution::letter_case_permutation(String::from("3z4")).into_iter().collect();
        assert_eq!(expected, actual);
    }
}

fn main() {
    println!("Hello, world!");
}
struct Solution;
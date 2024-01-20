impl Solution {
    pub fn split_words_by_separator(words: Vec<String>, separator: char) -> Vec<String> {
        words.iter().flat_map(|word| word.split(separator))
            .filter(|word| !word.is_empty())
            .map(|word| word.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(vec!["one","two","three","four","five","six"], Solution::split_words_by_separator(
            vec!["one.two.three","four.five","six"].iter().map(|x| x.to_string()).collect::<Vec<_>>(), '.'
        ));
    }

    #[test]
    fn example2() {
        assert_eq!(vec!["easy","problem"], Solution::split_words_by_separator(
            vec!["$easy$","$problem$"].iter().map(|x| x.to_string()).collect::<Vec<_>>(), '$'
        ));
    }

    #[test]
    fn example3() {
        let expected: Vec<String> = vec![];
        assert_eq!(expected, Solution::split_words_by_separator(
            vec!["|||"].iter().map(|x| x.to_string()).collect(), '|'
        ));
    }
}

fn main() {
    println!("Hello, world!");
}
struct Solution;
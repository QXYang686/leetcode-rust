use std::collections::HashMap;

impl Solution {
    pub fn count_words(words1: Vec<String>, words2: Vec<String>) -> i32 {
        let mut ans = 0;

        let (mut map1, mut map2) = (HashMap::new(), HashMap::new());

        words1.iter().for_each(|word| {
            *map1.entry(word).or_insert(0) += 1;
        });
        words2.iter().for_each(|word| {
            *map2.entry(word).or_insert(0) += 1;
        });

        map1.keys().for_each(|&word| {
            let &count1 = map1.get(word).unwrap_or(&-1);
            let &count2 = map2.get(word).unwrap_or(&-1);
            if count1 == 1 && count2 == 1 {
                ans += 1;
            }
        });
        ans
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(2, Solution::count_words(
            vec!["leetcode","is","amazing","as","is"].iter().map(|&word| word.to_string()).collect(),
            vec!["amazing","leetcode","is"].iter().map(|&word| word.to_string()).collect()
        ));
    }
    #[test]
    fn example2() {
        assert_eq!(0, Solution::count_words(
            vec!["b","bb","bbb"].iter().map(|&word| word.to_string()).collect(),
            vec!["a","aa","aaa"].iter().map(|&word| word.to_string()).collect()
        ));
    }
    #[test]
    fn example3() {
        assert_eq!(1, Solution::count_words(
            vec!["a","ab"].iter().map(|&word| word.to_string()).collect(),
            vec!["a","a","a","ab"].iter().map(|&word| word.to_string()).collect()
        ));
    }
}

fn main() {
    println!("Hello, world!");
}
struct Solution;
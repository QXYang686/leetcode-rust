use std::cmp::min;
use std::collections::BinaryHeap;

impl Solution {
    pub fn repeat_limited_string(s: String, repeat_limit: i32) -> String {
        // there is no itertools crate in Leetcode
        // let mut char_count = BinaryHeap::from_iter(s.chars()
        //     .into_iter().sorted().into_iter().group_by(|&char| char)
        //     .into_iter().map(|(char, group)| (char, group.count() as i32)));

        let mut char_count = BinaryHeap::new();

        let mut char_count_map = vec![0; 256];
        s.chars().for_each(|char| char_count_map[char as usize] += 1);
        &char_count_map['a' as usize ..= 'z' as usize].iter().enumerate()
            .filter(|entry| *entry.1 > 0)
            .for_each(|entry| {
                char_count.push(((entry.0 + 'a' as usize) as u8 as char, *entry.1));
            });

        let mut ans = String::with_capacity(s.chars().count());

        let mut unused_char = ('\0', 0);
        while !char_count.is_empty() {
            // println!("usused_char: {:?}, char_count: {:?}", unused_char, char_count);
            let (char, count) = char_count.pop().unwrap();
            // nothing left, use as much as possible
            if unused_char.1 <= 0 {
                let used_count = min(count, repeat_limit);
                for _ in 0..used_count {
                    ans.push(char);
                }
                unused_char = (char, count - used_count);
            }
            // something left, use single one and push_back unused
            else {
                ans.push(char);
                if count > 1 {
                    char_count.push((char, count - 1));
                }
                char_count.push(unused_char);
                unused_char.1 = 0;
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
        assert_eq!("zzcccac", Solution::repeat_limited_string("cczazcc".to_string(), 3));
    }
    #[test]
    fn example2() {
        assert_eq!("bbabaa", Solution::repeat_limited_string("aababab".to_string(), 2));
    }
}


fn main() {
    println!("Hello, world!");
}
struct Solution;
impl Solution {
    pub fn kth_grammar(n: i32, k: i32) -> i32 {
        if n == 1 && k == 1 {
            return 0;
        }
        else {
            return match (k % 2, Self::kth_grammar(n - 1, (k + 1) / 2)) {
                (0, 0) => 1,
                (0, 1) => 0,
                (1, 0) => 0,
                (1, 1) => 1,
                _ => panic!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(0, Solution::kth_grammar(1, 1));
    }
    #[test]
    fn example2() {
        assert_eq!(0, Solution::kth_grammar(2, 1));
    }
    #[test]
    fn example3() {
        assert_eq!(1, Solution::kth_grammar(2, 2));
    }
}

fn main() {
    println!("Hello, world!");
}
struct Solution;

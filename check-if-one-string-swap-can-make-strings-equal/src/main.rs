impl Solution {
    pub fn are_almost_equal(s1: String, s2: String) -> bool {
        let diff: Vec<_> = s1.chars().zip(s2.chars()).filter(|(x1, x2)| *x1 != *x2).collect();
        match diff.len() {
            0 => true,
            // equal after change
            2 if (diff[0].0, diff[1].0) == (diff[1].1, diff[0].1) => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(true, Solution::are_almost_equal(String::from("bank"), String::from("kanb")));
    }

    #[test]
    fn example2() {
        assert_eq!(false, Solution::are_almost_equal(String::from("attack"), String::from("defend")));
    }

    #[test]
    fn example3() {
        assert_eq!(true, Solution::are_almost_equal(String::from("kelb"), String::from("kelb")));
    }

    #[test]
    fn example4() {
        assert_eq!(false, Solution::are_almost_equal(String::from("abcd"), String::from("dcba")));
    }
}

fn main() {
    println!("Hello, world!");
}

struct Solution;
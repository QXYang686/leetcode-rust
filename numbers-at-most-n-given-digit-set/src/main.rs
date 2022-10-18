use std::cmp::Ordering;

impl Solution {
    pub fn at_most_n_given_digit_set(digits: Vec<String>, n: i32) -> i32 {
        let mut digits: Vec<i32> = digits.iter()
            .map(|x| x.parse().unwrap()).collect();
        digits.sort_unstable();
        let n_digits: Vec<i32> = n.to_string().chars()
            .map(|x| x.to_digit(10).unwrap() as i32).collect();

        let mut result = 0;
        for x in 1..n_digits.len() as u32 {
            result += digits.len().pow(x);
        }
        let mut result = result as i32;
        println!("{}", result);

        for (times, &n_digit) in n_digits.iter().rev().enumerate().rev() {
            for (index, &digit) in digits.iter().enumerate() {
                println!("{},{},{}", times, n_digit, digit);
                match digit.cmp(&n_digit) {
                    Ordering::Greater => {
                        return result;
                    }
                    Ordering::Equal => {
                        break;
                    }
                    Ordering::Less => {
                        result += digits.len().pow(times as u32) as i32;
                        if index == digits.len() - 1 {
                            return result;
                        }
                    }
                }
            }
        }
        result + 1
    }
}

mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(20, Solution::at_most_n_given_digit_set(vec![1, 3, 5, 7].iter().map(|x| x.to_string()).collect(), 100));
    }

    #[test]
    fn example2() {
        assert_eq!(29523, Solution::at_most_n_given_digit_set(vec![1, 4, 9].iter().map(|x| x.to_string()).collect(), 1000000000));
    }

    #[test]
    fn example3() {
        assert_eq!(1, Solution::at_most_n_given_digit_set(vec![7].iter().map(|x| x.to_string()).collect(), 8));
    }
}

fn main() {
    println!("Hello, world!");
}

struct Solution;
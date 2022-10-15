impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut next = 1;
        let mut result = vec![];
        for x in target {
            while x >= next {
                result.push(String::from("Push"));
                if x != next {
                    result.push(String::from("Pop"));
                }
                next += 1;
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
        assert_eq!(vec![String::from("Push"), String::from("Push"), String::from("Pop"), String::from("Push")],
                   Solution::build_array(vec![1, 3], 3))
    }

    #[test]
    fn example2() {
        assert_eq!(vec![String::from("Push"), String::from("Push"), String::from("Push")],
                   Solution::build_array(vec![1, 2, 3], 3))
    }

    #[test]
    fn example3() {
        assert_eq!(vec![String::from("Push"), String::from("Push")],
                   Solution::build_array(vec![1, 2], 4))
    }
}

fn main() {
    println!("Hello, world!");
}

struct Solution;
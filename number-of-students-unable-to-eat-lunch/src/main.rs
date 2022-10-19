impl Solution {
    pub fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
        let zero_count = students.iter().filter(|&x| *x == 0).count();
        let mut wants = [zero_count, students.len() - zero_count];
        for x in sandwiches {
            if wants[x as usize] == 0 {
                return (wants[0] + wants[1]) as i32;
            } else {
                wants[x as usize] -= 1;
            }
        }
        0
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(0, Solution::count_students(vec![1, 1, 0, 0], vec![0, 1, 0, 1]));
    }

    #[test]
    fn example2() {
        assert_eq!(3, Solution::count_students(vec![1, 1, 1, 0, 0, 1], vec![1, 0, 0, 0, 1, 1]));
    }
}

fn main() {
    println!("Hello, world!");
}

struct Solution;
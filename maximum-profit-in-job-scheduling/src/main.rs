impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        let mut data: Vec<_> = (0..start_time.len())
            .map(|x| (start_time[x], end_time[x], profit[x]))
            .collect();
        data.sort_unstable_by_key(|x| x.1);
        let mut results = vec![0; data.len() + 1];
        for i in 0..data.len() {
            let time_partition_point = data[..i].partition_point(|x| x.1 <= data[i].0);
            results[i + 1] = results[i].max(results[time_partition_point] + data[i].2);
        }
        *results.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(120, Solution::job_scheduling(vec![1, 2, 3, 3], vec![3, 4, 5, 6], vec![50, 10, 40, 70]));
    }

    #[test]
    fn example2() {
        assert_eq!(150, Solution::job_scheduling(vec![1, 2, 3, 4, 6], vec![3, 5, 10, 6, 9], vec![20, 20, 100, 70, 60]));
    }

    #[test]
    fn example3() {
        assert_eq!(6, Solution::job_scheduling(vec![1, 1, 1], vec![2, 3, 4], vec![5, 6, 4]));
    }
}

fn main() {
    println!("Hello, world!");
}

struct Solution;
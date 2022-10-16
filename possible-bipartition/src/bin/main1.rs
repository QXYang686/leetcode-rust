impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        // build graph
        let mut graph = vec![vec![]; n as usize + 1];
        for dislike in dislikes.iter() {
            graph[dislike[0] as usize].push(dislike[1]);
            graph[dislike[1] as usize].push(dislike[0]);
        }
        // Check circles
        let mut visited = vec![0; n as usize + 1];
        for dislike in dislikes.iter() {
            if visited[dislike[0] as usize] == 0 && partition_failed(&graph, &mut visited, dislike[0], 1) {
                return false;
            }
        }
        true
    }
}

fn partition_failed(graph: &Vec<Vec<i32>>, visited: &mut Vec<i32>, node: i32, depth: i32) -> bool {
    if visited[node as usize] != 0 {
        // circle found
        if (depth - visited[node as usize]) % 2 != 0 {
            return true;
        }
    } else {
        // circle not found, go deeper
        visited[node as usize] = depth;
        for next_node in graph[node as usize].iter() {
            if partition_failed(graph, visited, *next_node, depth + 1) {
                return true;
            }
        }
        // we will never visit a node twice, so the 'rollback' operation is unnecessary.
    }
    false
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn example1() {
        assert_eq!(true, Solution::possible_bipartition(4, vec![vec![1, 2], vec![1, 3], vec![2, 4]]));
    }

    #[test]
    fn example2() {
        assert_eq!(false, Solution::possible_bipartition(3, vec![vec![1, 2], vec![1, 3], vec![2, 3]]));
    }

    #[test]
    fn example3() {
        assert_eq!(false, Solution::possible_bipartition(5, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]]));
    }
}

fn main() {
    println!("Hello, world!");
}

struct Solution;
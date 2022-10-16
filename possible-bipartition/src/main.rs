use std::collections::{HashMap, HashSet};

impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        // build graph
        let mut graph = HashMap::new();
        for x in dislikes.iter() {
            let (from, to) = (x[0], x[1]);
            // from
            if !graph.contains_key(&from) {
                graph.insert(from, HashSet::new());
            }
            let neighbors_from = graph.get_mut(&from).unwrap();
            neighbors_from.insert(to);
            // to
            if !graph.contains_key(&to) {
                graph.insert(to, HashSet::new());
            }
            let neighbors_to = graph.get_mut(&to).unwrap();
            neighbors_to.insert(from);
        }
        // Check circles
        let mut visited = vec![false; n as usize + 1];
        let mut node_depth = HashMap::new();
        for dislike in dislikes {
            if !visited[dislike[0] as usize] && partition_failed(&graph, &mut visited, &mut node_depth, dislike[0], 0) {
                return false;
            }
        }
        true
    }
}

fn partition_failed(graph: &HashMap<i32, HashSet<i32>>, visited: &mut Vec<bool>, node_depth: &mut HashMap<i32, i32>, node: i32, depth: i32) -> bool {
    // circle found
    if visited[node as usize] {
        // circle size is even, failed situation detected
        if (depth - node_depth.get(&node).unwrap()) % 2 != 0 {
            return true;
        }
        // circle found, but no problem, do nothing and try another way in caller
    }
    // circle not found, we can go deeper
    else {
        visited[node as usize] = true;
        node_depth.insert(node, depth);
        for to in graph.get(&node).unwrap() {
            if partition_failed(graph, visited, node_depth, *to, depth + 1) {
                return true;
            }
        }
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
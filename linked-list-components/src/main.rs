use std::collections::HashSet;

impl Solution {
    pub fn num_components(head: Option<Box<ListNode>>, nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let num_set: HashSet<_> = nums.iter().collect();
        let mut head = &head;
        let mut exists = false;
        while let Some(node) = head {
            // a new components is found
            if !exists && num_set.contains(&node.val) {
                result += 1;
            }
            exists = num_set.contains(&node.val);
            head = &head.as_ref().unwrap().next;
        }
        result
    }
}


struct Solution;
fn main() {
    println!("Hello, world!");
}

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}
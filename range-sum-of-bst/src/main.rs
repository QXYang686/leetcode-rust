// Definition for a binary tree node.
#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode {
      val,
      left: None,
      right: None
    }
  }
}

use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
        match root {
            None => { 0 }
            Some(root) => {
                let mut root = root.borrow_mut();
                match root.val {
                    val if val < low => Self::range_sum_bst(root.right.take(), low, high),
                    val if val > high => Self::range_sum_bst(root.left.take(), low, high),
                    val => Self::range_sum_bst(root.left.take(), low, high) + val + Self::range_sum_bst(root.right.take(), low, high)
                }
            }
        }
    }
}

mod tests {
    
}

fn main() {
    println!("Hello, world!");
}
struct Solution;
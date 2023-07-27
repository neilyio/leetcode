// Category: algorithms
// Level: Easy
// Percent: 54.751225%

// Given the root of a binary tree, check whether it is a mirror of itself (i.e., symmetric around its center).
//
//
// Example 1:
//
// Input: root = [1,2,2,3,4,4,3]
// Output: true
//
//
// Example 2:
//
// Input: root = [1,2,2,null,3,null,3]
// Output: false
//
//
//
// Constraints:
//
//
// 	The number of nodes in the tree is in the range [1, 1000].
// 	-100 <= Node.val <= 100
//
//
//
// Follow up: Could you solve it both recursively and iteratively?
#![allow(dead_code)]
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
            right: None,
        }
    }
}

struct Solution {}
// @leetcode_start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::cell::RefCell;
use std::rc::Rc;
impl Solution {
    fn is_mirrored(
        left: Option<Rc<RefCell<TreeNode>>>,
        right: Option<Rc<RefCell<TreeNode>>>,
    ) -> bool {
        match (left, right) {
            (None, None) => true,
            (Some(l), Some(r)) => {
                let lb = l.borrow();
                let rb = r.borrow();

                lb.val == rb.val
                    && Solution::is_mirrored(lb.left.clone(), rb.right.clone())
                    && Solution::is_mirrored(lb.right.clone(), rb.left.clone())
            }
            _ => false,
        }
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        if let Some(node) = root {
            let bn = node.borrow();
            let left = &bn.left;
            let right = &bn.right;

            Solution::is_mirrored(left.clone(), right.clone())
        } else {
            true
        }
    }
}
// @leetcode_end
fn main() {}

// Category: algorithms
// Level: Easy
// Percent: 74.17827%

// Given the root of a binary tree, return its maximum depth.
//
// A binary tree's maximum depth is the number of nodes along the longest path from the root node down to the farthest leaf node.
//
//
// Example 1:
//
// Input: root = [3,9,20,null,null,15,7]
// Output: 3
//
//
// Example 2:
//
// Input: root = [1,null,2]
// Output: 2
//
//
//
// Constraints:
//
//
// 	The number of nodes in the tree is in the range [0, 10⁴].
// 	-100 <= Node.val <= 100
//
#![allow(dead_code)]
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
use std::cmp::max;
use std::rc::Rc;

impl Solution {
    fn max(root: Option<Rc<RefCell<TreeNode>>>, count: i32) -> i32 {
        if let Some(node_ref) = root {
            let node = node_ref.borrow();
            max(
                Solution::max(node.left.clone(), count + 1),
                Solution::max(node.right.clone(), count + 1),
            )
        } else {
            count
        }
    }
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::max(root, 0)
    }
}
// @leetcode_end
fn main() {}

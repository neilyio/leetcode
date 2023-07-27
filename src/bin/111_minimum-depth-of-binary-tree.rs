// Category: algorithms
// Level: Easy
// Percent: 44.93913%

// Given a binary tree, find its minimum depth.
//
// The minimum depth is the number of nodes along the shortest path from the root node down to the nearest leaf node.
//
// Note: A leaf is a node with no children.
//
//
// Example 1:
//
// Input: root = [3,9,20,null,null,15,7]
// Output: 2
//
//
// Example 2:
//
// Input: root = [2,null,3,null,4,null,5,null,6]
// Output: 5
//
//
//
// Constraints:
//
//
// 	The number of nodes in the tree is in the range [0, 10⁵].
// 	-1000 <= Node.val <= 1000
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
use std::rc::Rc;
impl Solution {
    fn recurse(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node_ref) = root {
            let node = node_ref.borrow();
            let is_leaf = node.left.is_some() && node.right.is_some();
            if is_leaf {
                1
            } else {
                
            }
            1 + Solution::recurse(node.left.clone()).min(Solution::recurse(node.right.clone()))
        } else {
            0
        }
    }
    pub fn min_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        Solution::recurse(root)
    }
}
// @leetcode_end
fn main() {}
c
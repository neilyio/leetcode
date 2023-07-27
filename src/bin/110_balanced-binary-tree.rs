// Category: algorithms
// Level: Easy
// Percent: 49.691765%

// Given a binary tree, determine if it is height-balanced.
//
//
// Example 1:
//
// Input: root = [3,9,20,null,null,15,7]
// Output: true
//
//
// Example 2:
//
// Input: root = [1,2,2,3,3,null,null,4,4]
// Output: false
//
//
// Example 3:
//
// Input: root = []
// Output: true
//
//
//
// Constraints:
//
//
// 	The number of nodes in the tree is in the range [0, 5000].
// 	-10⁴ <= Node.val <= 10⁴
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
    fn recurse(root: Option<Rc<RefCell<TreeNode>>>) -> (bool, i32) {
        if let Some(node_ref) = root {
            let node = node_ref.borrow();
            let left = node.left.clone();
            let right = node.right.clone();
            let (left_balanced, left_height) = Solution::recurse(left);
            let (right_balanced, right_height) = Solution::recurse(right);
            (
                (left_balanced && right_balanced && (left_height - right_height).abs() <= 1),
                1 + left_height.max(right_height),
            )
        } else {
            (true, 0)
        }
    }

    pub fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        Solution::recurse(root).0
    }
}
// @leetcode_end
fn main() {}

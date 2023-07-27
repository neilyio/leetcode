// Category: algorithms
// Level: Easy
// Percent: 74.25736%

// Given the root of a binary tree, return the inorder traversal of its nodes' values.
//
//
// Example 1:
//
// Input: root = [1,null,2,3]
// Output: [1,3,2]
//
//
// Example 2:
//
// Input: root = []
// Output: []
//
//
// Example 3:
//
// Input: root = [1]
// Output: [1]
//
//
//
// Constraints:
//
//
// 	The number of nodes in the tree is in the range [0, 100].
// 	-100 <= Node.val <= 100
//
//
//
// Follow up: Recursive solution is trivial, could you do it iteratively?
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
    fn traverse(root: Option<Rc<RefCell<TreeNode>>>, result: &mut Vec<i32>) {
        if let Some(node) = root {
            let n = node.borrow();
            Solution::traverse(n.left.clone(), result);
            result.push(n.val);
            Solution::traverse(n.right.clone(), result);
        }
    }
    pub fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        Solution::traverse(root, &mut result);

        return result;
    }
}
// @leetcode_end
fn main() {}

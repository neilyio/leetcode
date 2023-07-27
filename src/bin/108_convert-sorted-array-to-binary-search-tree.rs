// Category: algorithms
// Level: Easy
// Percent: 70.25354%

// Given an integer array nums where the elements are sorted in ascending order, convert it to a height-balanced binary search tree.
//
//
// Example 1:
//
// Input: nums = [-10,-3,0,5,9]
// Output: [0,-3,9,-10,null,5]
// Explanation: [0,-10,5,null,-3,null,9] is also accepted:
//
//
//
// Example 2:
//
// Input: nums = [1,3]
// Output: [3,1]
// Explanation: [1,null,3] and [3,1] are both height-balanced BSTs.
//
//
//
// Constraints:
//
//
// 	1 <= nums.length <= 10⁴
// 	-10⁴ <= nums[i] <= 10⁴
// 	nums is sorted in a strictly increasing order.
//
#![allow(dead_code)]
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
    fn to_bst(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
        if nums.len() == 0 {
            return None;
        }
        if nums.len() == 1 {
            return Some(Rc::new(RefCell::new(TreeNode::new(nums[0]))));
        }

        let mid = nums.len() / 2;
        let node = TreeNode {
            val: nums[mid],
            left: Solution::to_bst(&nums[0..mid]),
            right: Solution::to_bst(&nums[(mid + 1)..nums.len()]),
        };

        Some(Rc::new(RefCell::new(node)))
    }
    pub fn sorted_array_to_bst(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        Solution::to_bst(&nums)
    }
}
// @leetcode_end
fn main() {}

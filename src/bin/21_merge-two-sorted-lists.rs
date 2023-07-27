// Category: algorithms
// Level: Easy
// Percent: 62.838573%

// You are given the heads of two sorted linked lists list1 and list2.
//
// Merge the two lists in a one sorted list. The list should be made by splicing together the nodes of the first two lists.
//
// Return the head of the merged linked list.
//
//
// Example 1:
//
// Input: list1 = [1,2,4], list2 = [1,3,4]
// Output: [1,1,2,3,4,4]
//
//
// Example 2:
//
// Input: list1 = [], list2 = []
// Output: []
//
//
// Example 3:
//
// Input: list1 = [], list2 = [0]
// Output: [0]
//
//
//
// Constraints:
//
//
// 	The number of nodes in both lists is in the range [0, 50].
// 	-100 <= Node.val <= 100
// 	Both list1 and list2 are sorted in non-decreasing order.
//
#![allow(dead_code)]

pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}
// @leetcode_start
// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }

// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(Box::new(ListNode {
                val: n.val,
                next: Solution::merge_two_lists(n.next, None),
            })),
            (Some(n1), Some(n2)) => {
                if n1.val < n2.val {
                    Some(Box::new(ListNode {
                        val: n1.val,
                        next: Solution::merge_two_lists(n1.next, Some(n2)),
                    }))
                } else {
                    Some(Box::new(ListNode {
                        val: n2.val,
                        next: Solution::merge_two_lists(n2.next, Some(n1)),
                    }))
                }
            }
        }
    }
}
// @leetcode_end
fn main() {}

// Category: algorithms
// Level: Easy
// Percent: 50.976078%

// Given the head of a sorted linked list, delete all duplicates such that each element appears only once. Return the linked list sorted as well.
//
//
// Example 1:
//
// Input: head = [1,1,2]
// Output: [1,2]
//
//
// Example 2:
//
// Input: head = [1,1,2,3,3]
// Output: [1,2,3]
//
//
//
// Constraints:
//
//
// 	The number of nodes in the list is in the range [0, 300].
// 	-100 <= Node.val <= 100
// 	The list is guaranteed to be sorted in ascending order.
//
#![allow(dead_code)]

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
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
//
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
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut curr_opt = head.as_mut();

        while let Some(curr) = curr_opt {
            while let Some(next) = curr.next.take() {
                if curr.val == next.val {
                    curr.next = next.next;
                } else {
                    curr.next = Some(next);
                    break;
                }
            }
            curr_opt = curr.next.as_mut();
        }

        head
    }
}
// @leetcode_end
fn main() {}

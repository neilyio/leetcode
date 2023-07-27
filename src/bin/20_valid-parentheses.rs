// Category: algorithms
// Level: Easy
// Percent: 40.258293%

// Given a string s containing just the characters '(', ')', '{', '}', '[' and ']', determine if the input string is valid.
//
// An input string is valid if:
//
//
// 	Open brackets must be closed by the same type of brackets.
// 	Open brackets must be closed in the correct order.
// 	Every close bracket has a corresponding open bracket of the same type.
//
//
//
// Example 1:
//
// Input: s = "()"
// Output: true
//
//
// Example 2:
//
// Input: s = "()[]{}"
// Output: true
//
//
// Example 3:
//
// Input: s = "(]"
// Output: false
//
//
//
// Constraints:
//
//
// 	1 <= s.length <= 10⁴
// 	s consists of parentheses only '()[]{}'.
//

#![allow(dead_code)]
struct Solution {}
// @leetcode_start
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];

        for char in s.chars() {
            match char {
                '(' | '[' | '{' => stack.push(char),
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                _ => unreachable!("only brackets expected"),
            }
        }

        stack.len() == 0
    }
}
// @leetcode_end
fn main() {}

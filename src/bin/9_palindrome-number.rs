// Category: algorithms
// Level: Easy
// Percent: 53.95101%

// Given an integer x, return true if x is a palindrome, and false otherwise.
//
//
// Example 1:
//
// Input: x = 121
// Output: true
// Explanation: 121 reads as 121 from left to right and from right to left.
//
//
// Example 2:
//
// Input: x = -121
// Output: false
// Explanation: From left to right, it reads -121. From right to left, it becomes 121-. Therefore it is not a palindrome.
//
//
// Example 3:
//
// Input: x = 10
// Output: false
// Explanation: Reads 01 from right to left. Therefore it is not a palindrome.
//
//
//
// Constraints:
//
//
// 	-2³¹ <= x <= 2³¹ - 1
//
//
//
// Follow up: Could you solve it without converting the integer to a string?

#![allow(dead_code)]
struct Solution {}
// @leetcode_start
impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let xs: Vec<_> = x.to_string().chars().collect();
        let mut lp = 0;
        let mut rp = xs.len() - 1;

        loop {
            if lp >= rp {
                return true;
            }

            if xs[lp] != xs[rp] {
                return false;
            }

            lp += 1;
            rp -= 1;
        }
    }
}
// @leetcode_end
fn main() {}

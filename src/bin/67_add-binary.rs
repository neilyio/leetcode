// Category: algorithms
// Level: Easy
// Percent: 52.481068%

// Given two binary strings a and b, return their sum as a binary string.
//
//
// Example 1:
// Input: a = "11", b = "1"
// Output: "100"
// Example 2:
// Input: a = "1010", b = "1011"
// Output: "10101"
//
//
// Constraints:
//
//
// 	1 <= a.length, b.length <= 10⁴
// 	a and b consist only of '0' or '1' characters.
// 	Each string does not contain leading zeros except for the zero itself.
//

#![allow(dead_code)]
struct Solution {}
// @leetcode_start

impl Solution {
    pub fn add_binary(mut a: String, mut b: String) -> String {
        if a.len() < b.len() {
            std::mem::swap(&mut a, &mut b);
        }

        let mut carry = 0;
        let mut a = a.into_bytes();
        let mut b = b.bytes().rev();

        for a_byte in a.iter_mut().rev() {
            if let Some(b_byte) = b.next() {
                *a_byte += b_byte - b'0';
            }

            *a_byte += carry;

            if *a_byte - b'0' > 1 {
                carry = 1;
                *a_byte = b'0' + *a_byte % 2;
            } else {
                carry = 0
            }
        }

        if carry > 0 {
            a.insert(0, b'1')
        }

        unsafe { String::from_utf8_unchecked(a) }
    }
}
// @leetcode_end
fn main() {}

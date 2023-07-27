// Category: algorithms
// Level: Easy
// Percent: 37.621593%

// Given a non-negative integer x, return the square root of x rounded down to the nearest integer. The returned integer should be non-negative as well.
//
// You must not use any built-in exponent function or operator.
//
//
// 	For example, do not use pow(x, 0.5) in c++ or x ** 0.5 in python.
//
//
//
// Example 1:
//
// Input: x = 4
// Output: 2
// Explanation: The square root of 4 is 2, so we return 2.
//
//
// Example 2:
//
// Input: x = 8
// Output: 2
// Explanation: The square root of 8 is 2.82842..., and since we round it down to the nearest integer, 2 is returned.
//
//
//
// Constraints:
//
//
// 	0 <= x <= 2³¹ - 1
//

#![allow(dead_code)]
struct Solution {}
// @leetcode_start
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        let x = x as f64;
        let mut guess = x;

        if guess == 0.0 {
            return 0;
        }

        loop {
            let new = 0.5 * (guess + (x / guess));

            if guess - new <= 0.00001 {
                return new as i32;
            }

            guess = new;
        }
    }
}
// @leetcode_end
fn main() {}

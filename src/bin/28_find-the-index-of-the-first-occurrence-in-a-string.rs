// Category: algorithms
// Level: Easy
// Percent: 39.754646%

// Given two strings needle and haystack, return the index of the first occurrence of needle in haystack, or -1 if needle is not part of haystack.
//
//
// Example 1:
//
// Input: haystack = "sadbutsad", needle = "sad"
// Output: 0
// Explanation: "sad" occurs at index 0 and 6.
// The first occurrence is at index 0, so we return 0.
//
//
// Example 2:
//
// Input: haystack = "leetcode", needle = "leeto"
// Output: -1
// Explanation: "leeto" did not occur in "leetcode", so we return -1.
//
//
//
// Constraints:
//
//
// 	1 <= haystack.length, needle.length <= 10⁴
// 	haystack and needle consist of only lowercase English characters.
//

#![allow(dead_code)]
struct Solution {}
// @leetcode_start
impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let hchars: Vec<char> = haystack.chars().collect();
        let nchars: Vec<char> = needle.chars().collect();
        let mut lp: usize = 0;

        loop {
            if lp >= hchars.len() {
                return -1;
            }

            if hchars[lp] != nchars[0] {
                lp += 1;
                continue;
            }

            let mut n_idx = nchars.len() - 1;
            let mut rp = lp + n_idx;
            if rp >= hchars.len() {
                return -1;
            }

            while hchars[rp] == nchars[n_idx] {
                if rp == lp {
                    return lp as i32;
                }

                rp -= 1;
                n_idx -= 1;
            }

            lp += 1;
        }
    }
}
// @leetcode_end
fn main() {}

// Category: algorithms
// Level: Easy
// Percent: 44.382824%

// Given a string s consisting of words and spaces, return the length of the last word in the string.
//
// A word is a maximal substring consisting of non-space characters only.
//
//
// Example 1:
//
// Input: s = "Hello World"
// Output: 5
// Explanation: The last word is "World" with length 5.
//
//
// Example 2:
//
// Input: s = "   fly me   to   the moon  "
// Output: 4
// Explanation: The last word is "moon" with length 4.
//
//
// Example 3:
//
// Input: s = "luffy is still joyboy"
// Output: 6
// Explanation: The last word is "joyboy" with length 6.
//
//
//
// Constraints:
//
//
// 	1 <= s.length <= 10⁴
// 	s consists of only English letters and spaces ' '.
// 	There will be at least one word in s.
//

#![allow(dead_code)]
struct Solution {}
// @leetcode_start
impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut reversed = s.bytes().rev();
        let mut count = 0;

        loop {
            match reversed.next() {
                None => return count,
                Some(c) if c == b' ' => {
                    if count > 0 {
                        return count;
                    }
                }
                _ => count += 1,
            }
        }
    }
}
// @leetcode_end
fn main() {}

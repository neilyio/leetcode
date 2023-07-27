// Category: algorithms
// Level: Easy
// Percent: 41.08187%

// Write a function to find the longest common prefix string amongst an array of strings.
//
// If there is no common prefix, return an empty string "".
//
//
// Example 1:
//
// Input: strs = ["flower","flow","flight"]
// Output: "fl"
//
//
// Example 2:
//
// Input: strs = ["dog","racecar","car"]
// Output: ""
// Explanation: There is no common prefix among the input strings.
//
//
//
// Constraints:
//
//
// 	1 <= strs.length <= 200
// 	0 <= strs[i].length <= 200
// 	strs[i] consists of only lowercase English letters.
//

#![allow(dead_code)]
struct Solution {}
// @leetcode_start

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut shortest_len = strs[0].len();
        let mut prefix: Vec<char> = vec![];

        for s in &strs {
            for (idx, char) in s.chars().enumerate() {
                match &prefix.get(idx) {
                    Some(&pc) if pc == char => continue,
                    Some(_) => {
                        if idx < shortest_len {
                            shortest_len = idx;
                        }
                    }
                    None => prefix.push(char),
                }
            }

            if s.len() < shortest_len {
                shortest_len = s.len();
            }
        }

        prefix.truncate(shortest_len);
        prefix.into_iter().collect::<String>()
    }
}
// @leetcode_end
fn main() {}

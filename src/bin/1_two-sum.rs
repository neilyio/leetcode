// Category: algorithms
// Level: Easy
// Percent: 50.154865%

// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.
//
// You may assume that each input would have exactly one solution, and you may not use the same element twice.
//
// You can return the answer in any order.
//
//
// Example 1:
//
// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1].
//
//
// Example 2:
//
// Input: nums = [3,2,4], target = 6
// Output: [1,2]
//
//
// Example 3:
//
// Input: nums = [3,3], target = 6
// Output: [0,1]
//
//
//
// Constraints:
//
//
// 	2 <= nums.length <= 10⁴
// 	-10⁹ <= nums[i] <= 10⁹
// 	-10⁹ <= target <= 10⁹
// 	Only one valid answer exists.
//
//
//
// Follow-up: Can you come up with an algorithm that is less than O(n²) time complexity?

#![allow(dead_code)]
struct Solution {}
// @leetcode_start
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut indexed = nums.into_iter().enumerate().collect::<Vec<(usize, i32)>>();

        indexed.sort_unstable_by_key(|&(_, n)| n);

        let mut lp = 0;
        let mut rp = &indexed.len() - 1;

        loop {
            let (li, ln) = indexed[lp];
            let (ri, rn) = indexed[rp];
            let sum = ln + rn;

            if sum == target {
                return vec![li as i32, ri as i32];
            }

            if sum > target {
                rp -= 1;
            }

            if sum < target {
                lp += 1;
            }
        }
    }
}
// @leetcode_end
fn main() {}

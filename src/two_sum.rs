pub struct Solution;

use std::collections::HashMap;

#[allow(dead_code)]
impl Solution {
    pub fn two_sum_array(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for i in 0..nums.len() {
            for j in (i + 1)..nums.len() {
                let num_a = nums[i];
                let num_b = nums[j];
                if num_b == target - num_a {
                    return vec![i as i32, j as i32];
                }
            }
        }
        vec![]
    }

    pub fn two_sum_hmap(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hmap = HashMap::<i32, usize>::new();
        for index in 0..nums.len() {
            if let Some(key_index) = hmap.get(&(target - nums[index])) {
                return vec![index as i32, *key_index as i32];
            } else {
                hmap.insert(nums[index], index);
            }
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(Solution::two_sum_hmap(nums, target), vec![1, 0]);
    }
}

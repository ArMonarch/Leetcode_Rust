pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        Solution::method_2(nums)
    }

    /// Using HashMap
    fn method_1(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        if nums.is_empty() {
            return 0;
        }

        let mut hmap = HashMap::<i32, i32>::new();
        let mut result = 0;
        let mut majority = 0;

        for num in nums {
            hmap.entry(num).and_modify(|count| *count += 1).or_insert(1);

            // never panics in hmap.get because the num will always be availale in the hmap by
            // above line.
            if *hmap.get(&num).unwrap() > majority {
                result = num;
                majority = *hmap.get(&num).unwrap();
            }
        }

        return result;
    }

    /// Moore Voting Algorithm
    fn method_2(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        let mut majority = 0;

        for num in nums {
            if majority == 0 {
                result = num;
            }

            if num == result {
                majority += 1;
            } else {
                majority -= 1;
            }
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_01() {
        let nums = vec![3, 2, 3];
        let expected = 3;

        assert_eq!(expected, Solution::majority_element(nums));
    }

    #[test]
    fn case_02() {
        let nums = vec![2, 2, 1, 1, 1, 2, 2];
        let expected = 2;

        assert_eq!(expected, Solution::majority_element(nums));
    }

    #[test]
    fn case_03() {
        let nums = vec![];
        let expected = 0;

        assert_eq!(expected, Solution::majority_element(nums));
    }

    #[test]
    fn case_04() {
        let nums = vec![1, 1, 1, 1, 1];
        let expected = 1;

        assert_eq!(expected, Solution::majority_element(nums));
    }
}

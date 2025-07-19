pub struct Solution;

use std::collections::HashSet;
#[allow(dead_code)]
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut single_number = HashSet::<i32>::new();

        for number in nums {
            if single_number.contains(&number) {
                assert_eq!(single_number.remove(&number), true);
            } else {
                assert_eq!(single_number.insert(number), true);
            }
        }

        return single_number.into_iter().next().unwrap();
    }

    pub fn single_number_xor(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for number in nums {
            result = result ^ number;
        }

        return result;
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn case_1() {
        let input = vec![2, 2, 1];
        let output = 1;

        assert_eq!(Solution::single_number_xor(input), output);
    }

    #[test]
    fn case_2() {
        let input = vec![4, 1, 2, 1, 2];
        let output = 4;

        assert_eq!(Solution::single_number_xor(input), output);
    }

    #[test]
    fn case_3() {
        let input = vec![1];
        let output = 1;

        assert_eq!(Solution::single_number_xor(input), output);
    }
}

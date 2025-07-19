struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut index_1 = 0 as usize;

        for index_2 in 0..nums.len() {
            if nums[index_2] != val {
                nums.swap(index_1, index_2);
                index_1 += 1;
            }
        }

        return index_1 as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_01() {
        let mut input = vec![3, 2, 2, 3];
        let output = vec![2, 2, 3, 3];

        assert_eq!(2, Solution::remove_element(&mut input, 3));

        assert_eq!(output, input);
    }

    #[test]
    fn case_02() {
        let mut input = vec![0, 1, 2, 2, 3, 0, 4, 2];
        let output = vec![0, 1, 3, 0, 4, 2, 2, 2];

        assert_eq!(5, Solution::remove_element(&mut input, 2));

        assert_eq!(output, input);
    }

    #[test]
    fn case_03() {
        let mut input = vec![];
        let output: Vec<i32> = vec![];

        assert_eq!(0, Solution::remove_element(&mut input, 0));

        assert_eq!(output, input);
    }
}

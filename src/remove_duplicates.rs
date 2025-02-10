pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.is_empty() {
            return 0;
        }

        let mut count_index = 0;
        for i in 1..nums.len() {
            if nums[count_index] != nums[i] {
                count_index += 1;
                nums[count_index] = nums[i];
            }
        }

        return (count_index + 1) as i32;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let mut input = vec![1, 1, 2];
        let output = vec![1, 2];

        let k = Solution::remove_duplicates(&mut input);
        assert_eq!(k as usize, output.len());

        for i in 0..k as usize {
            assert_eq!(input[i], output[i]);
        }
    }

    #[test]
    fn case_2() {
        let mut input = vec![0, 1, 1, 1, 2, 2, 3, 3, 4];
        let output = vec![0, 1, 2, 3, 4];

        let k = Solution::remove_duplicates(&mut input);
        assert_eq!(k as usize, output.len());

        for i in 0..k as usize {
            assert_eq!(input[i], output[i]);
        }
    }
}

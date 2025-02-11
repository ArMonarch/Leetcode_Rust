pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn missing_number(nums: Vec<i32>) -> i32 {
        if nums.len() == 0 {
            return -1;
        }

        let nums_sum: i32 = nums.iter().sum();
        let range_sum: i32 = (0..=nums.len() as i32).sum();

        return range_sum - nums_sum;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let input = vec![3, 0, 1];
        let output = 2;

        assert_eq!(Solution::missing_number(input), output);
    }

    #[test]
    fn case_2() {
        let input = vec![0, 1];
        let output = 2;

        assert_eq!(Solution::missing_number(input), output);
    }

    #[test]
    fn case_3() {
        let input = vec![9, 6, 4, 2, 3, 5, 7, 0, 1];
        let output = 8;

        assert_eq!(Solution::missing_number(input), output);
    }

    #[test]
    fn case_4() {
        let input = vec![3, 0, 1];
        let output = 2;

        assert_eq!(Solution::missing_number(input), output);
    }

    #[test]
    fn case_5() {
        let input = vec![3, 0, 1];
        let output = 2;

        assert_eq!(Solution::missing_number(input), output);
    }
}

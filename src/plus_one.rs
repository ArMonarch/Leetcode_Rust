pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn plus_one(mut digit: Vec<i32>) -> Vec<i32> {
        for value in digit.iter_mut().rev() {
            if *value < 9 {
                *value += 1;
                return digit;
            } else {
                *value = 0;
            }
        }
        digit.insert(0, 1);
        return digit;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let input = vec![1, 2, 3];
        let output = vec![1, 2, 4];

        assert_eq!(Solution::plus_one(input), output);
    }

    #[test]
    fn case_2() {
        let input = vec![4, 3, 2, 1];
        let output = vec![4, 3, 2, 2];

        assert_eq!(Solution::plus_one(input), output);
    }

    #[test]
    fn case_3() {
        let input = vec![9];
        let output = vec![1, 0];

        assert_eq!(Solution::plus_one(input), output);
    }

    #[test]
    fn case_4() {
        let input = vec![9, 9, 9, 9];
        let output = vec![1, 0, 0, 0, 0];

        assert_eq!(Solution::plus_one(input), output);
    }
}

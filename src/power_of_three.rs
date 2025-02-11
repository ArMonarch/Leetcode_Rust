pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_power_of_three(num: i32) -> bool {
        match num {
            _ if num < 0 => return false,
            _ if num == 1 => return true,
            _ => {}
        }
        // as negative value of num is already handeled, num can safely be cased to u32
        for index in 1..(num / 3 + 1) as u32 {
            let exp_value = 3_u32.pow(index);
            if exp_value == num as u32 {
                return true;
            } else if exp_value > num as u32 {
                return false;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let input = 3;
        let output = true;

        assert_eq!(Solution::is_power_of_three(input), output);
    }

    #[test]
    fn case_2() {
        let input = 27;
        let output = true;

        assert_eq!(Solution::is_power_of_three(input), output);
    }

    #[test]
    fn case_3() {
        let input = -1;
        let output = false;

        assert_eq!(Solution::is_power_of_three(input), output);
    }
}

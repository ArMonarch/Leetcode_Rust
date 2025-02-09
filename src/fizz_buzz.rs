pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn fizz_buzz(num: i32) -> Vec<String> {
        let mut result = Vec::<String>::with_capacity(num as usize);
        for i in 1..=num {
            match i {
                _ if (i % 3 == 0 && i % 5 == 0) => result.push("FizzBuzz".to_string()),
                _ if (i % 3 == 0) => result.push("Fizz".to_string()),
                _ if (i % 5 == 0) => result.push("Buzz".to_string()),
                _ => result.push(format!("{}", i)),
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let input = 3;
        let output = Solution::fizz_buzz(input);

        assert_eq!(output, vec!["1", "2", "Fizz"]);
    }

    #[test]
    fn case_2() {
        let input = 5;
        let output = Solution::fizz_buzz(input);

        assert_eq!(output, vec!["1", "2", "Fizz", "4", "Buzz"]);
    }

    #[test]
    fn case_3() {
        let input = 15;
        let output = Solution::fizz_buzz(input);

        assert_eq!(
            output,
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}

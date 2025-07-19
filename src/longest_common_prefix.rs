use std::iter::zip;

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_common_prefix(mut strs: Vec<String>) -> String {
        if strs.len() == 0 {
            return String::new();
        }

        if strs.len() == 1 {
            return strs.pop().unwrap();
        }

        let mut result = String::new();

        strs.sort();

        let (first, last) = (&strs[0], &strs[strs.len() - 1]);

        for (a, b) in zip(first.chars(), last.chars()) {
            if a != b {
                return result;
            }

            result.push(a);
        }

        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let input = vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string(),
        ];
        let output = "fl".to_string();

        assert_eq!(Solution::longest_common_prefix(input), output);
    }

    #[test]
    fn case_2() {
        let input = vec!["dog".to_string(), "racecar".to_string(), "car".to_string()];
        let output = "".to_string();

        assert_eq!(Solution::longest_common_prefix(input), output);
    }
}

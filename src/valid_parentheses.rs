pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid(str: String) -> bool {
        if str.len() % 2 != 0 {
            return false;
        }

        let mut stack = Vec::<char>::new();

        for char in str.chars() {
            match char {
                '(' => stack.push(char),
                '{' => stack.push(char),
                '[' => stack.push(char),
                ']' => {
                    if let Some('[') = stack.pop() {
                        continue;
                    } else {
                        return false;
                    }
                }
                '}' => {
                    if let Some('{') = stack.pop() {
                        continue;
                    } else {
                        return false;
                    }
                }
                ')' => {
                    if let Some('(') = stack.pop() {
                        continue;
                    } else {
                        return false;
                    }
                }
                _ => return false,
            }
        }

        if stack.len() != 0 {
            return false;
        } else {
            return true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_01() {
        let input = "()".to_string();

        let output = true;

        assert_eq!(Solution::is_valid(input), output);
    }

    #[test]
    fn case_02() {
        let input = "({})".to_string();

        let output = true;

        assert_eq!(Solution::is_valid(input), output);
    }

    #[test]
    fn case_03() {
        let input = "(){}[]".to_string();

        let output = true;

        assert_eq!(Solution::is_valid(input), output);
    }

    #[test]
    fn case_04() {
        let input = "((()".to_string();

        let output = false;

        assert_eq!(Solution::is_valid(input), output);
    }
}

use std::collections::HashMap;

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn roman_to_int(string: String) -> i32 {
        let mut roman_value = HashMap::<char, i32>::new();
        roman_value.insert(char::from('I'), 1);
        roman_value.insert(char::from('V'), 5);
        roman_value.insert(char::from('X'), 10);
        roman_value.insert(char::from('L'), 50);
        roman_value.insert(char::from('C'), 100);
        roman_value.insert(char::from('D'), 500);
        roman_value.insert(char::from('M'), 1000);

        let chars: Vec<char> = string.chars().collect();

        let mut result: i32 = 0;

        // convert to roman_value to int for range 0 .. string.len -1 && add last roman_value after the loop
        for index in 0..string.len() - 1 {
            let str_current = &chars[index];
            let str_next = &chars[index + 1];

            if roman_value.get(str_current).unwrap() < roman_value.get(str_next).unwrap() {
                result -= roman_value.get(str_current).unwrap();
            } else {
                result += roman_value.get(str_current).unwrap();
            }
        }

        result += roman_value.get(&chars[string.len() - 1]).unwrap();
        return result;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let input = String::from("III");
        let output = 3;
        assert_eq!(Solution::roman_to_int(input), output);
    }

    #[test]
    fn case_2() {
        let input = String::from("LVIII");
        let output = 58;
        assert_eq!(Solution::roman_to_int(input), output);
    }

    #[test]
    fn case_3() {
        let input = String::from("MCMXCIV");
        let output = 1994;
        assert_eq!(Solution::roman_to_int(input), output);
    }
}

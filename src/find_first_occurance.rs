pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn find_frist_occurance(haystack: String, needle: String) -> i32 {
        if needle.len() > haystack.len() {
            return -1;
        }
        for index in 0..=haystack.len() - needle.len() {
            if haystack
                .get(index..(index + needle.len()))
                .unwrap()
                .contains(&needle)
            {
                return index as i32;
            }
        }
        return -1;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let haystack = String::from("sadbutsad");
        let needle = String::from("sad");

        let output = 0;

        assert_eq!(Solution::find_frist_occurance(haystack, needle), output);
    }

    #[test]
    fn case_2() {
        let haystack = String::from("leetcode");
        let needle = String::from("leeto");

        let output = -1;

        assert_eq!(Solution::find_frist_occurance(haystack, needle), output);
    }

    #[test]
    fn case_3() {
        let haystack = String::from("hello");
        let needle = String::from("o");

        let output = 4;

        assert_eq!(Solution::find_frist_occurance(haystack, needle), output);
    }
}

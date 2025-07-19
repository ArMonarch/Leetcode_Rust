pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn reverse_string(chars: &mut Vec<char>) {
        let string_len = chars.len();

        for index in 0..string_len / 2 {
            chars.swap(index, string_len - 1 - index);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let mut input = vec!['h', 'e', 'l', 'l', 'o'];
        let output = vec!['o', 'l', 'l', 'e', 'h'];
        Solution::reverse_string(&mut input);
        assert_eq!(input, output);
    }

    #[test]
    fn case_2() {
        let mut input = vec!['h', 'a', 'n', 'n', 'a', 'h'];
        let output = vec!['h', 'a', 'n', 'n', 'a', 'h'];
        Solution::reverse_string(&mut input);
        assert_eq!(input, output);
    }
}

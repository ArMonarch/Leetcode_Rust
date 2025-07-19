pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn hamming_weight(num: i32) -> i32 {
        (0..32).fold(0, |acc, exp| acc + ((num & (1 << exp)) >> exp)) as i32
    }
}

#[cfg(test)]
mod tests {

    use super::Solution;

    #[test]
    fn case_1() {
        let input = 11;
        let output = 3;

        assert_eq!(Solution::hamming_weight(input), output);
    }

    #[test]
    fn case_2() {
        let input = 128;
        let output = 1;

        assert_eq!(Solution::hamming_weight(input), output);
    }

    #[test]
    fn case_3() {
        let input = 2147483645;
        let output = 30;

        assert_eq!(Solution::hamming_weight(input), output);
    }
}

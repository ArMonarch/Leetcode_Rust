pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn length_of_last_word(str: String) -> i32 {
        str.trim().split(" ").last().unwrap().len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_01() {
        let str = String::from("Hello World");
        let expected = 5;

        assert_eq!(expected, Solution::length_of_last_word(str));
    }

    #[test]
    fn case_02() {
        let str = String::from("   fly me   to   the moon  ");
        let expected = 4;

        assert_eq!(expected, Solution::length_of_last_word(str));
    }

    #[test]
    fn case_03() {
        let str = String::from("luffy is still joyboy");
        let expected = 6;

        assert_eq!(expected, Solution::length_of_last_word(str));
    }

    #[test]
    fn case_04() {
        let str = String::from("   luffy   ");
        let expected = 5;

        assert_eq!(expected, Solution::length_of_last_word(str));
    }
}

use std::cmp::Ordering;

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn sqrt(num: i32) -> i32 {
        if num == 0 || num == 1 {
            return num;
        }

        let mut start = 0;
        let mut end = num;
        let mut mid;
        while start <= end {
            mid = start + (end - start) / 2;

            let square: u64 = mid as u64 * mid as u64;
            match square.cmp(&(num as u64)) {
                Ordering::Less => start = mid + 1,
                Ordering::Equal => return mid,
                Ordering::Greater => end = mid - 1,
            }
        }
        return end;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let input = 4;
        let output = 2;
        assert_eq!(Solution::sqrt(input), output);
    }

    #[test]
    fn case_2() {
        let input = 9;
        let output = 3;
        assert_eq!(Solution::sqrt(input), output);
    }

    #[test]
    fn case_3() {
        let input = 16;
        let output = 4;
        assert_eq!(Solution::sqrt(input), output);
    }

    #[test]
    fn case_4() {
        let input = 0;
        let output = 0;
        assert_eq!(Solution::sqrt(input), output);
    }

    #[test]
    fn case_5() {
        let input = 2147395599;
        let output = 46339;
        assert_eq!(Solution::sqrt(input), output);
    }

    #[test]
    fn case_6() {
        let input = 8;
        let output = 2;
        assert_eq!(Solution::sqrt(input), output);
    }
}

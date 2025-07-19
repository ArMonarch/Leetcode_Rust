pub struct Solution;

#[allow(dead_code)]
impl Solution {
    /// 1 -> 1
    /// 2 -> 2
    /// 3 -> 3
    /// 4 -> 5
    /// 5 -> 8
    /// 6 -> 13
    /// 7 -> 21
    /// This is an Fibonachi series,
    /// Fibonachi series : 0,1,2,3,5,8,13,21
    /// constrains: 1 <= n <= 45
    pub fn climb_stairs(n: i32) -> i32 {
        return Solution::method_02(n);
    }

    /// Recursion Method
    pub fn method_01(num: i32) -> i32 {
        if num == 2 || num == 1 || num == 0 {
            return num;
        }

        return Solution::method_01(num - 1) + Solution::method_01(num - 2);
    }

    /// Iterative dynamic programming method
    pub fn method_02(num: i32) -> i32 {
        assert!(1 <= num && num <= 45);

        let mut memo = vec![1, 2];

        if num == 2 || num == 1 {
            return num;
        }

        for i in 2..num as usize {
            let a = memo[i - 1];
            let b = memo[i - 2];

            memo.push(a + b);
        }

        assert!(memo.len() > 0);

        return memo.pop().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_01() {
        let input = 1;
        let output = 1;

        assert_eq!(Solution::climb_stairs(input), output);
    }

    #[test]
    fn case_02() {
        let input = 2;
        let output = 2;

        assert_eq!(Solution::climb_stairs(input), output);
    }

    #[test]
    fn case_03() {
        let input = 3;
        let output = 3;

        assert_eq!(Solution::climb_stairs(input), output);
    }

    #[test]
    fn case_04() {
        let input = 4;
        let output = 5;

        assert_eq!(Solution::climb_stairs(input), output);
    }

    #[test]
    fn case_05() {
        let input = 5;
        let output = 8;

        assert_eq!(Solution::climb_stairs(input), output);
    }

    #[test]
    fn case_06() {
        let input = 6;
        let output = 13;

        assert_eq!(Solution::climb_stairs(input), output);
    }

    #[test]
    fn case_07() {
        let input = 7;
        let output = 21;

        assert_eq!(Solution::climb_stairs(input), output);
    }

    #[test]
    fn case_08() {
        let input = 8;
        let output = 34;

        assert_eq!(Solution::climb_stairs(input), output);
    }

    #[test]
    fn case_09() {
        let input = 9;
        let output = 55;

        assert_eq!(Solution::climb_stairs(input), output);
    }

    #[test]
    fn case_10() {
        let input = 10;
        let output = 89;

        assert_eq!(Solution::climb_stairs(input), output);
    }
}

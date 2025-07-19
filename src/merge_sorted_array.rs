pub struct Solution;

#[allow(dead_code, unused_variables)]
impl Solution {
    pub fn merge(nums_1: &mut Vec<i32>, m: i32, nums_2: &mut Vec<i32>, n: i32) {
        Solution::method_01(nums_1, m, nums_2, n);
    }

    fn method_01(nums_1: &mut Vec<i32>, m: i32, nums_2: &mut Vec<i32>, n: i32) {
        let mut midx = m - 1; // points to the last element in the meaningful part of nums_1 
        let mut right = m + n - 1; // points to the last index in nums_1
        let mut nidx = n - 1; // points to the last element in nums_2

        while nidx >= 0 {
            if midx >= 0 && nums_1[midx as usize] > nums_2[nidx as usize] {
                nums_1[right as usize] = nums_1[midx as usize];
                midx -= 1;
            } else {
                nums_1[right as usize] = nums_2[nidx as usize];
                nidx -= 1;
            }

            right -= 1;
        }
    }

    // append nums_2 to  nums_1, and sort the resulting array
    fn method_02(nums_1: &mut Vec<i32>, m: i32, nums_2: &mut Vec<i32>, n: i32) {
        let mut i = m as usize;
        let mut j = 0;

        while i < nums_1.len() {
            nums_1[i] = nums_2[j];

            i += 1;
            j += 1;
        }

        nums_1.sort();
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_01() {
        let mut nums_1 = vec![0, 1, 5, 9, 0, 0, 0];
        let mut nums_2 = vec![0, 2, 3];

        let m = 4;
        let n = 3;

        let output = vec![0, 0, 1, 2, 3, 5, 9];

        Solution::merge(&mut nums_1, m, &mut nums_2, n);

        assert_eq!(nums_1, output);
    }

    #[test]
    fn case_02() {
        let mut nums_1 = vec![1, 2, 3, 0, 0, 0];
        let mut nums_2 = vec![2, 5, 6];

        let m = 3;
        let n = 3;

        let output = vec![1, 2, 2, 3, 5, 6];

        Solution::merge(&mut nums_1, m, &mut nums_2, n);

        assert_eq!(nums_1, output);
    }

    #[test]
    fn case_03() {
        let mut nums_1 = vec![0, 0];
        let mut nums_2 = vec![0];

        let m = 1;
        let n = 1;

        let output = vec![0, 0];

        Solution::merge(&mut nums_1, m, &mut nums_2, n);

        assert_eq!(nums_1, output);
    }

    #[test]
    fn case_04() {
        let mut nums_1 = vec![0];
        let mut nums_2 = vec![];

        let m = 1;
        let n = 0;

        let output = vec![0];

        Solution::merge(&mut nums_1, m, &mut nums_2, n);

        assert_eq!(nums_1, output);
    }

    #[test]
    fn case_05() {
        let mut nums_1 = vec![0, 0, 0, 0];
        let mut nums_2 = vec![0, 1, 2, 3];

        let m = 0;
        let n = 4;

        let output = vec![0, 1, 2, 3];

        Solution::merge(&mut nums_1, m, &mut nums_2, n);

        assert_eq!(nums_1, output);
    }
}

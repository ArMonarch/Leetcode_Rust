pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut pascal_traingle: Vec<Vec<i32>> = Vec::new();

        for i in 0..num_rows as usize {
            let mut row_values: Vec<i32> = Vec::with_capacity(i + 1);

            for j in 0..=i {
                if j == 0 || j == i {
                    row_values.push(1);
                } else {
                    row_values.push(pascal_traingle[i - 1][j - 1] + pascal_traingle[i - 1][j]);
                }
            }

            pascal_traingle.push(row_values);
        }

        return pascal_traingle;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_1() {
        let input = 5;
        println!("{:?}", Solution::generate(input));
    }

    #[test]
    fn case_2() {
        let input = 1;
        println!("{:?}", Solution::generate(input));
    }
}

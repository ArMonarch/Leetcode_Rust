pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut buy = prices[0];
        let mut profit = 0;

        for price in prices {
            if buy > price {
                buy = price;
            }

            if profit < (price - buy) {
                profit = price - buy;
            }
        }

        return profit;
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn case_01() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        let expected = 5;

        assert_eq!(expected, Solution::max_profit(prices));
    }

    #[test]
    fn case_02() {
        let prices = vec![7, 6, 4, 3, 1];
        let expected = 0;

        assert_eq!(expected, Solution::max_profit(prices));
    }

    #[test]
    fn case_03() {
        let prices = vec![7, 1, 5, 1, 7, 3, 10, 13];
        let expected = 12;

        assert_eq!(expected, Solution::max_profit(prices));
    }
}

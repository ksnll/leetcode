struct Solution;

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut solution = 0;
        for i in 0..prices.len() - 1 {
            let mut g = i + 1;
            while g < prices.len() && prices[g] - prices[i] > 0 {
                if solution < prices[g] - prices[i] {
                    solution = prices[g] - prices[i];
                }
                g = g + 1;
            }
        }
        solution
    }
}

fn main() {
    dbg!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]));
}

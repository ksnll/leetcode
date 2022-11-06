struct Solution;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let result = nums.into_iter().reduce(|a, b| a ^ b).unwrap();
        result
    }
}

fn main() {
    dbg!(Solution::single_number(vec![4, 1, 2, 1, 2]));
}

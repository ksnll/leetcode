struct Solution;
impl Solution {
    pub fn missing_number(mut nums: Vec<i32>) -> i32 {
        nums.sort();
        for i in 0..nums.len() {
            if i as i32 != nums[i] {
                return i as i32;
            }
        }
        (nums.len()) as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::missing_number(vec![3, 0, 1]);
        assert_eq!(result, 2);

        let result = Solution::missing_number(vec![0, 1]);
        assert_eq!(result, 2);

        let result = Solution::missing_number(vec![9, 6, 4, 2, 3, 5, 7, 0, 1]);
        assert_eq!(result, 8);
    }
}

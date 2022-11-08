struct Solution;
impl Solution {
    pub fn is_power_of_three(mut n: i32) -> bool {
        if n == 0 {
            return false;
        }
        if n == 1 {
            return true;
        }
        while n % 3 == 0 {
            if n <= 3 {
                return true;
            }
            n = n / 3;
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::is_power_of_three(27);
        assert_eq!(result, true);

        let result = Solution::is_power_of_three(0);
        assert_eq!(result, false);

        let result = Solution::is_power_of_three(2);
        assert_eq!(result, false);

        let result = Solution::is_power_of_three(1);
        assert_eq!(result, false);

        let result = Solution::is_power_of_three(3);
        assert_eq!(result, true);

        let result = Solution::is_power_of_three(-1);
        assert_eq!(result, false);
    }
}

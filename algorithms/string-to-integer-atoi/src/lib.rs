struct Solution;
impl Solution {
    pub fn my_atoi(s: String) -> i32 {
        let mut chars = s.chars().peekable();
        let mut solution: i32 = 0;
        let mut multiply = 1;

        while let Some(_) = chars.next_if(|x| *x == ' ') {}

        if let Some(sign) = chars.next_if(|x| *x == '+' || *x == '-') {
            if sign == '-' {
                multiply = -1;
            }
        }

        while let Some(char) = chars.next() {
            match char {
                '0'..='9' => {
                    let digit = char.to_digit(10).unwrap() as i32;
                    if let Some(newval) = solution.checked_mul(10) {
                        solution = newval;
                    } else {
                        if multiply > 0 {
                            return i32::MAX;
                        } else {
                            return i32::MIN;
                        }
                    }
                    if let Some(newval) = solution.checked_add(digit) {
                        solution = newval
                    } else {
                        if multiply > 0 {
                            return i32::MAX;
                        } else {
                            return i32::MIN;
                        }
                    };
                }
                _ => return solution as i32 * multiply,
            }
        }
        solution as i32 * multiply as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::my_atoi("42".to_owned());
        assert_eq!(result, 42);

        let result = Solution::my_atoi("-42".to_owned());
        assert_eq!(result, -42);

        let result = Solution::my_atoi("4193".to_owned());
        assert_eq!(result, 4193);

        let result = Solution::my_atoi("   -42".to_owned());
        assert_eq!(result, -42);

        let result = Solution::my_atoi("words and 987".to_owned());
        assert_eq!(result, 0);

        let result = Solution::my_atoi("-91283472332".to_owned());
        assert_eq!(result, -2147483648);

        let result = Solution::my_atoi("21474836460".to_owned());
        assert_eq!(result, 2147483647);

        let result = Solution::my_atoi("-21474836460".to_owned());
        assert_eq!(result, -2147483647)
    }
}

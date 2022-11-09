struct Solution;
impl Solution {
    pub fn fizz_buzz(n: i32) -> Vec<String> {
        let mut solution = Vec::new();
        for i in 1..n + 1 {
            if i % 5 == 0 && i % 3 == 0 {
                solution.push("FizzBuzz".to_owned());
                continue;
            }
            if i % 3 == 0 {
                solution.push("Fizz".to_owned());
                continue;
            }
            if i % 5 == 0 {
                solution.push("Buzz".to_owned());
                continue;
            }
            solution.push((i).to_string());
        }
        solution
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::fizz_buzz(3);
        assert_eq!(result, vec!["1", "2", "Fizz"]);

        let result = Solution::fizz_buzz(5);
        assert_eq!(result, vec!["1", "2", "Fizz", "4", "Buzz"]);

        let result = Solution::fizz_buzz(15);
        assert_eq!(
            result,
            vec![
                "1", "2", "Fizz", "4", "Buzz", "Fizz", "7", "8", "Fizz", "Buzz", "11", "Fizz",
                "13", "14", "FizzBuzz"
            ]
        );
    }
}

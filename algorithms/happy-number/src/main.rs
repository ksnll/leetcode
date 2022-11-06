struct Solution;

impl Solution {
    pub fn is_happy(n: i32) -> bool {
        let mut solution: i32 = n;
        loop {
            solution = solution
                .to_string()
                .chars()
                .map(|x| u32::pow(x.to_digit(10).unwrap(), 2))
                .reduce(|acc, item| acc + item)
                .unwrap() as i32;

            println!("{}", solution);
            if solution == 1 || solution == 7 {
                return true;
            }

            if solution.to_string().len() == 1 {
                return false;
            }
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn is_happy() {
        assert_eq!(true, crate::Solution::is_happy(19));
        assert_eq!(false, crate::Solution::is_happy(2));
    }
}

fn main() {
    dbg!(Solution::is_happy(1111111));
}

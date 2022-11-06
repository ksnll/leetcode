struct Solution;
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        let mut total = 0;
        for c in column_title.chars() {
            total = total * 26;
            total = total + (c as usize - 64);
        }
        total as i32
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn title_to_number() {
        assert_eq!(1, crate::Solution::title_to_number("A".to_owned()));
        assert_eq!(2, crate::Solution::title_to_number("B".to_owned()));
        assert_eq!(26, crate::Solution::title_to_number("Z".to_owned()));
        assert_eq!(28, crate::Solution::title_to_number("AB".to_owned()));
        assert_eq!(29, crate::Solution::title_to_number("AC".to_owned()));
        assert_eq!(701, crate::Solution::title_to_number("ZY".to_owned()));
    }
}

fn main() {
    dbg!(Solution::title_to_number("ZY".to_owned()));
}

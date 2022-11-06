struct Solution;

impl Solution {
    pub fn hammingWeight(n: u32) -> i32 {
        format!("{:b}", n)
            .chars()
            .map(|x| x.to_digit(2).unwrap())
            .reduce(|acc, item| acc + item)
            .unwrap() as i32
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn hamming_weight() {
        assert_eq!(1, crate::Solution::hammingWeight(4));
        assert_eq!(2, crate::Solution::hammingWeight(5));
    }
}

fn main() {
    dbg!(Solution::hammingWeight(43261596));
}

struct Solution;

impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        let bits_string = format!("{:b}", x);
        let pad_size = 32 - bits_string.len();
        let mut padding = (0..pad_size).map(|_| '0').collect::<String>();
        padding.push_str(&bits_string);
        let bits_string = padding.chars().rev().collect::<String>();
        let mut solution = 0;
        for (i, bit) in bits_string.chars().rev().enumerate() {
            solution += u32::pow(2, i as u32) * bit.to_digit(2).unwrap();
        }
        solution as u32
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn reverse_bits() {
        assert_eq!(964176192, crate::Solution::reverse_bits(43261596));
        assert_eq!(3221225471, crate::Solution::reverse_bits(4294967293));
    }
}

fn main() {
    dbg!(Solution::reverse_bits(43261596));
}

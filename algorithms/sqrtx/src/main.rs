struct Solution;
impl Solution {
    pub fn my_sqrt(x: i32) -> i32 {
        match x {
            0 => 0,
            1 => 1,
            _ => {
                let mut square: usize = 2;
                while square * square <= x as usize {
                    square += 1;
                }
                square as i32 - 1
            }
        }
    }
}

fn main() {
    dbg!(Solution::my_sqrt(2147395600));
}

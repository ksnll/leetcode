struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut sum_hashmap = std::collections::HashMap::new();
        for (ix, i) in nums.iter().enumerate() {
            sum_hashmap.insert(i, ix);
        }
        for (ix, i) in nums.iter().enumerate() {
            match sum_hashmap.get(&(target - i)) {
                Some(gx) => {
                    if ix == *gx {
                        continue;
                    } else {
                        return vec![ix as i32, *gx as i32];
                    }
                }
                None => continue,
            }
        }
        panic!("Not found")
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![2, 7, 11, 15], 9));
}

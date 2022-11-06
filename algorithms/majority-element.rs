use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut hashmap: HashMap<i32, usize> = HashMap::new();
        for i in nums.iter() {
            let value = hashmap.get(i).unwrap_or(&0) + 1;
            if value > nums.len() / 2 {
                return *i;
            }
            hashmap.insert(*i, value);
        }
        0
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn should_get_the_majority() {
        assert_eq!(3, crate::Solution::majority_element(vec![1, 3, 3, 3, 4]));
    }
}

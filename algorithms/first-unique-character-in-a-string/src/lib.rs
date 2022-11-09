use std::collections::HashMap;

struct Solution;
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let mut hashmap = HashMap::new();
        for char in s.chars() {
            let count = hashmap.get(&char).unwrap_or(&0);
            hashmap.insert(char, count + 1);
        }
        for (i, char) in s.chars().enumerate() {
            let count = hashmap.get(&char).unwrap_or(&0);
            if *count == 1 {
                return i as i32;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_unique_character() {
        let result = Solution::first_uniq_char("leetcode".to_owned());
        assert_eq!(result, 0);

        let result = Solution::first_uniq_char("loveleetcode".to_owned());
        assert_eq!(result, 2);

        let result = Solution::first_uniq_char("aabb".to_owned());
        assert_eq!(result, -1);
    }
}

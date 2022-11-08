struct Solution;
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut sorted_s = s.chars().collect::<Vec<char>>();
        let mut sorted_t = t.chars().collect::<Vec<char>>();
        sorted_t.sort();
        sorted_s.sort();
        sorted_s.into_iter().collect::<String>() == sorted_t.into_iter().collect::<String>()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = Solution::is_anagram(String::from("anagram"), String::from("nagaram"));
        assert_eq!(result, true);

        let result = Solution::is_anagram(String::from("rat"), String::from("car"));
        assert_eq!(result, false);
    }
}

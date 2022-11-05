struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        dbg!(s
            .to_lowercase()
            .chars()
            .rev()
            .collect::<String>()
            .replace(|c: char| !c.is_alphabetic(), ""));
        s.to_lowercase().replace(|c: char| !c.is_alphabetic(), "")
            == s.to_lowercase()
                .chars()
                .rev()
                .collect::<String>()
                .replace(|c: char| !c.is_alphabetic(), "")
    }
}

fn main() {
    dbg!(Solution::is_palindrome(String::from(
        "A man, a plan, a canal: Panama"
    )));
}

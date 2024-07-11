struct Solution {}

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let alphanumeric_s: Vec<String> = s
            .chars()
            .filter(|val| val.is_alphanumeric())
            .map(|val| val.to_lowercase().to_string())
            .collect();

        if alphanumeric_s.len() == 0 {
            return true;
        };

        let mut start_index = 0;
        let mut end_index = alphanumeric_s.len() - 1;

        while start_index < end_index {
            if alphanumeric_s[start_index] != alphanumeric_s[end_index] {
                return false;
            } else {
                start_index += 1;
                end_index -= 1;
            }
        }

        true
    }
}

#[test]
fn test_case_1() {
    let s = String::from("A man, a plan, a canal: Panama");
    assert_eq!(Solution::is_palindrome(s), true);
}

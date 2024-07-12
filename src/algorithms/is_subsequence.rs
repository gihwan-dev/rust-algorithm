struct Solution {}

impl Solution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s: Vec<char> = s.chars().collect();
        let t: Vec<char> = t.chars().collect();

        if s.len() > t.len() {
            return false;
        }

        if s.len() == t.len() {
            return s == t;
        }

        let mut t_index = 0;
        let mut s_index = 0;

        while s_index < s.len() && t_index < t.len() {
            if s[s_index] == t[t_index] {
                s_index += 1;
            }

            t_index += 1;
        }

        s_index == s.len()
    }
}

struct BestSolution {}

impl BestSolution {
    pub fn is_subsequence(s: String, t: String) -> bool {
        let mut iter = t.chars();

        for c in s.chars() {
            match iter.find(|&target| target == c) {
                Some(_) => (),
                None => return false,
            }
        }

        true
    }
}

#[test]
fn test_case_1() {
    let s = String::from("abc");
    let t = String::from("ahbgdc");

    assert_eq!(self::Solution::is_subsequence(s, t), true);
}

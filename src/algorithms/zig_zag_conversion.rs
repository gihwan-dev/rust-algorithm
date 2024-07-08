struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        let mut zigzags: Vec<_> = (0..num_rows)
            .chain((1..num_rows - 1).rev())
            .cycle()
            .zip(s.chars())
            .collect();
        zigzags.sort_by_key(|&(row, _)| row);
        zigzags.into_iter().map(|(_, c)| c).collect()
    }
}

#[test]
fn test_zig_zag_conversion() {
    let s = String::from("PAYPALISHIRING");
    let nums_rows = 3;
    let result = Solution::convert(s, nums_rows);
    assert_eq!(result, String::from("PAHNAPLSIIGYIR"));

    let s = String::from("PAYPALISHIRING");
    let nums_rows = 4;
    let result = Solution::convert(s, nums_rows);
    assert_eq!(result, String::from("PINALSIGYAHRPI"));
}

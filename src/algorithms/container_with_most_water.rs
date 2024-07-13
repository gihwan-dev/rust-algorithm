use std::cmp;

struct Solution {}

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut left = 0;
        let mut right = height.len() - 1;

        while left < right {
            let area = cmp::min(height[left], height[right]) * (right as i32 - left as i32);

            max_area = cmp::max(max_area, area);

            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_area
    }
}

struct BestSolution {}

impl BestSolution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut max_area = 0;
        let mut iter = height.iter().enumerate();
        let mut left = iter.next();
        let mut right = iter.next_back();

        while let (Some((left_index, left_height)), Some((right_index, right_height))) =
            (left, right)
        {
            max_area = cmp::max(
                max_area,
                cmp::min(left_height, right_height) * (right_index - left_index) as i32,
            );

            if left_height < right_height {
                left = iter.next();
            } else {
                right = iter.next_back();
            }
        }

        max_area
    }
}

use std::cmp::Ordering::{self, *};

struct Solution {}

impl Solution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = vec![];

        for (index, num) in numbers.iter().enumerate() {
            println!("num: {}, target - num: {}", num, target - num);
            if let Some(target_index) = numbers[index + 1..].iter().position(|&x| x == target - num)
            {
                result.push(index as i32 + 1);
                result.push(index as i32 + 1 + target_index as i32 + 1);
                break;
            }
        }

        result
    }
}

struct BestSolution {}

impl BestSolution {
    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut start = 0;
        let mut end = 0;

        while start < end {
            match (start + end).cmp(&target) {
                Ordering::Less => start += 1,
                Ordering::Greater => end -= 1,
                Ordering::Equal => return vec![(start + 1) as i32, (end + 1) as i32],
            }
        }
        unreachable!();
    }
}

#[test]
fn test_case_1() {
    let numbers = vec![2, 7, 11, 15];
    let target = 9;

    assert_eq!(self::Solution::two_sum(numbers, target), vec![1, 2]);
}

use crate::Runnable;
use std::fs;

pub struct Solution;

impl Solution {
    pub fn solve(input: &str) -> u32 {
        let mut ans = 0;
        for i in 0..input.len() - 1 {
            let a = input.chars().nth(i).unwrap();
            let b = input.chars().nth(i+1).unwrap();
            if a == b {
                ans += a.to_digit(10).unwrap();
            }
        }
        let a = input.chars().nth(input.len() - 1).unwrap();
        let b = input.chars().nth(0).unwrap();
        if a == b {
            ans += a.to_digit(10).unwrap();
        }
        ans
    }
}

impl Runnable for Solution {
    fn run() {
        let contents =
            fs::read_to_string("src/2017/day1/part1/input.txt").expect("Failed to read input");
        for line in contents.lines() {
            let result = Solution::solve(line);
            println!("{:?}", result);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        assert_eq!(Solution::solve("1122"), 3);
        assert_eq!(Solution::solve("1111"), 4);
        assert_eq!(Solution::solve("1234"), 0);
        assert_eq!(Solution::solve("91212129"), 9);
    }
}

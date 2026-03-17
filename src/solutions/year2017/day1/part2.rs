use crate::Runnable;
use std::fs;

pub struct Solution;

impl Solution {
    pub fn solve(input: &str) -> u32 {
        let mut ans = 0;
        let size = input.len();
        let mid = input.len() / 2;
        for i in 0..size {
            let a = input.chars().nth(i).unwrap();
            let b = input.chars().nth((i + mid) % size).unwrap();
            if a == b {
                ans += a.to_digit(10).unwrap();
            }
        }
        ans
    }
}

impl Runnable for Solution {
    fn run() {
        let contents =
            fs::read_to_string("src/2017/day1/part2/input.txt").expect("Failed to read input");
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
    fn test_1212() {
        assert_eq!(Solution::solve("1212"), 6);
    }

    #[test]
    fn test_1221() {
        assert_eq!(Solution::solve("1221"), 0);
    }

    #[test]
    fn test_123425() {
        assert_eq!(Solution::solve("123425"), 4);
    }

    #[test]
    fn test_123123() {
        assert_eq!(Solution::solve("123123"), 12);
    }

    #[test]
    fn test_12131415() {
        assert_eq!(Solution::solve("12131415"), 4);
    }
}

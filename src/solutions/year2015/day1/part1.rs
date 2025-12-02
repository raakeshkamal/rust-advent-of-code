use std::fs;
use crate::Runnable;

pub struct Solution;

impl Solution {
    pub fn solve(input: &str) -> i32 {
        let byte_str = input.as_bytes();
        let mut floor = 0;
        for i in byte_str {
            if *i as char == '(' {
                floor += 1;
            } else if *i as char == ')' {
                floor -= 1;
            }
        }
        floor
    }
}

impl Runnable for Solution {
    fn run() {
        let contents = fs::read_to_string("src/2015/day1/part1/input.txt").expect("Failed to read input");
        let result = Solution::solve(&contents);
        println!("{:?}", result);
    }
}

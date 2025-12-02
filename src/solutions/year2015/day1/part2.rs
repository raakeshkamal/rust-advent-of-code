use std::fs;
use crate::Runnable;

pub struct Solution;

impl Solution {
    pub fn solve(input: &str) -> usize {
        let byte_str = input.as_bytes();
        let mut floor = 0;
        let mut pos = 0;
        for i in byte_str {
            if *i as char == '(' {
                floor += 1;
            } else if *i as char == ')' {
                floor -= 1;
            }
            pos += 1;
            if floor < 0 {
                return pos;
            }
        }
        0 // Should not reach here
    }
}

impl Runnable for Solution {
    fn run() {
        let contents = fs::read_to_string("src/2015/day1/part2/input.txt").expect("Failed to read input");
        let result = Solution::solve(&contents);
        println!("{:?}", result);
    }
}

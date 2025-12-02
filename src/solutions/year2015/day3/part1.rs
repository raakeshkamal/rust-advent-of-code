use std::collections::HashMap;
use std::fs;
use crate::Runnable;

pub struct Solution;

impl Solution {
    pub fn solve(input: &str) -> usize {
        let mut gifts: HashMap<(i32, i32), u32> = HashMap::new();
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        gifts.insert((x, y), 1);
        let byte_str = input.as_bytes();
        for i in byte_str {
            let temp = *i as char;
            if temp == '>' {
                x += 1;
            } else if temp == '^' {
                y += 1;
            } else if temp == 'v' {
                y -= 1;
            } else if temp == '<' {
                x -= 1;
            }
            let location = (x, y);
            *gifts.entry(location).or_insert(0) += 1;
        }
        gifts.len()
    }
}

impl Runnable for Solution {
    fn run() {
        let contents = fs::read_to_string("src/2015/day3/part1/input.txt").expect("Failed to read input");
        let result = Solution::solve(&contents);
        println!("{:?}", result);
    }
}

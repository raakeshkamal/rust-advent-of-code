use std::fs;
use crate::Runnable;

pub struct Solution;

impl Solution {
    pub fn solve(input: &str) -> u64 {
        let box_list: Vec<String> = input.lines().map(String::from).collect();
        let mut ribbon: u64 = 0;
        for str in box_list {
            let mut dimensions: Vec<u64> = str.split("x").map(|x| x.parse::<u64>().unwrap()).collect();
            dimensions.sort();
            ribbon += 2 * (dimensions[0] + dimensions[1]);
            ribbon += dimensions[0] * dimensions[1] * dimensions[2];
        }
        ribbon
    }
}

impl Runnable for Solution {
    fn run() {
        let contents = fs::read_to_string("src/2015/day2/part2/input.txt").expect("Failed to read input");
        let result = Solution::solve(&contents);
        println!("{:?}", result);
    }
}

use std::fs;
use crate::Runnable;

pub struct Solution;

impl Solution {
    pub fn solve(input: &str) -> u64 {
        let box_list: Vec<String> = input.lines().map(String::from).collect();
        let mut area: u64 = 0;
        for str in box_list {
            let mut dimensions: Vec<u64> = str.split("x").map(|x| x.parse::<u64>().unwrap()).collect();
            dimensions.sort();
            area += 3 * dimensions[0] * dimensions[1];
            area += 2 * dimensions[1] * dimensions[2];
            area += 2 * dimensions[0] * dimensions[2];
        }
        area
    }
}

impl Runnable for Solution {
    fn run() {
        let contents = fs::read_to_string("src/2015/day2/part1/input.txt").expect("Failed to read input");
        let result = Solution::solve(&contents);
        println!("{:?}", result);
    }
}

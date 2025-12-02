use std::collections::HashMap;
use std::fs;
use crate::Runnable;

pub struct Solution;

impl Solution {
    fn is_nice(s: &str) -> bool {
        let mut has_repeat_separated = false;
        let mut has_pair_twice = false;
        
        // Check for letter repeated with one in between (i and i+2 same)
        for i in 0..s.len().saturating_sub(2) {
            if s.as_bytes()[i] == s.as_bytes()[i + 2] {
                has_repeat_separated = true;
                break;
            }
        }
        
        // Check for pair that appears at least twice
        let mut pair_pos: HashMap<String, Vec<usize>> = HashMap::new();
        for i in 0..s.len().saturating_sub(1) {
            let pair: String = s[i..i + 2].to_string();
            pair_pos.entry(pair).or_insert_with(Vec::new).push(i);
        }
        
        for positions in pair_pos.values() {
            if positions.len() >= 2 {
                let first = positions[0];
                let second = *positions.iter().skip(1).min_by_key(|&&p| (p - first).abs()).unwrap_or(&0);
                if (second - first).abs() > 1 {
                    has_pair_twice = true;
                    break;
                }
            }
        }
        
        has_repeat_separated && has_pair_twice
    }

    pub fn solve(input: &str) -> u32 {
        let str_list: Vec<&str> = input.lines().collect();
        let mut nice: u32 = 0;
        for str in str_list {
            if Self::is_nice(str) {
                nice += 1;
            }
        }
        nice
    }
}

impl Runnable for Solution {
    fn run() {
        let contents = fs::read_to_string("src/2015/day5/part2/input.txt").expect("Failed to read input");
        let result = Solution::solve(&contents);
        println!("{:?}", result);
    }
}

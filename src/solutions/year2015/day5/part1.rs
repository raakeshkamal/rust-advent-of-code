use std::fs;
use crate::Runnable;

pub struct Solution;

impl Solution {
    fn is_nice(s: &str) -> bool {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let bad_subs = ["ab", "cd", "pq", "xy"];
        
        let mut vowel_count = 0;
        let mut has_double = false;
        let mut has_bad = false;
        
        let mut prev_char = None;
        for c in s.chars() {
            if vowels.contains(&c) {
                vowel_count += 1;
            }
            if let Some(prev) = prev_char {
                if prev == c {
                    has_double = true;
                }
                let sub: String = [prev, c].iter().collect();
                if bad_subs.contains(&sub.as_str()) {
                    has_bad = true;
                }
            }
            prev_char = Some(c);
        }
        
        vowel_count >= 3 && has_double && !has_bad
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
        let contents = fs::read_to_string("src/2015/day5/part1/input.txt").expect("Failed to read input");
        let result = Solution::solve(&contents);
        println!("{:?}", result);
    }
}

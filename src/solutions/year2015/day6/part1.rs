use regex::{Captures, Regex};
use std::fs;
use crate::Runnable;

pub struct Solution;

#[derive(Debug)]
struct Instruction {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
    i: char,
}

impl Solution {
    fn parse_string(s: String) -> Instruction {
        let re_on = Regex::new(r"turn on ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)").unwrap();
        let re_off = Regex::new(r"turn off ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)").unwrap();
        let re_toggle = Regex::new(r"toggle ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)").unwrap();
        
        let caps = if let Some(c) = re_on.captures(&s) {
            ('s', c)
        } else if let Some(c) = re_off.captures(&s) {
            ('r', c)
        } else if let Some(c) = re_toggle.captures(&s) {
            ('t', c)
        } else {
            panic!("Invalid instruction: {}", s);
        };
        
        Instruction {
            x1: caps.1[1].parse().unwrap(),
            y1: caps.1[2].parse().unwrap(),
            x2: caps.1[3].parse().unwrap(),
            y2: caps.1[4].parse().unwrap(),
            i: caps.0,
        }
    }

    pub fn solve(input: &str) -> u64 {
        let str_list: Vec<String> = input.lines().map(String::from).collect();
        let mut lights: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];
        
        for str in str_list {
            let ins = Self::parse_string(str);
            for x in ins.x1..=ins.x2 {
                for y in ins.y1..=ins.y2 {
                    match ins.i {
                        's' => lights[x as usize][y as usize] += 1,
                        'r' => {
                            if lights[x as usize][y as usize] > 0 {
                                lights[x as usize][y as usize] -= 1;
                            }
                        }
                        't' => lights[x as usize][y as usize] += 2,
                        _ => panic!(),
                    }
                }
            }
        }
        
        let mut num: u64 = 0;
        for i in 0..1000 {
            for j in 0..1000 {
                num += lights[i][j] as u64;
            }
        }
        num
    }
}

impl Runnable for Solution {
    fn run() {
        let contents = fs::read_to_string("src/2015/day6/input.txt").expect("Failed to read input");
        let result = Solution::solve(&contents);
        println!("{:?}", result);
    }
}

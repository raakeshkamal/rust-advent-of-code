use md5;
use std::fs;
use crate::Runnable;

pub struct Solution;

impl Solution {
    fn is_good_hash(hash: &str) -> bool {
        hash.starts_with("000000")
    }

    pub fn solve(secret: &[u8]) -> u32 {
        let mut i: u32 = 0;
        loop {
            let temp = i.to_string();
            let test = temp.as_bytes();
            let res = [secret, test].concat();
            let out = format!("{:X}", md5::compute(res));
            if Self::is_good_hash(&out) {
                return i;
            }
            i += 1;
        }
    }
}

impl Runnable for Solution {
    fn run() {
        let contents = fs::read_to_string("src/2015/day4/input.txt").expect("Failed to read input");
        let secret = contents.trim().as_bytes();
        let result = Solution::solve(secret);
        println!("{:?}", result);
    }
}

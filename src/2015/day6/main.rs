use regex::{Captures, Regex};
use std::fs;

fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

struct Instruction {
    x1: u16,
    y1: u16,
    x2: u16,
    y2: u16,
    i: char,
}

fn parse_string(str: String) -> Instruction {
    let mut ins: Instruction = Instruction {
        x1: 0,
        y1: 0,
        x2: 0,
        y2: 0,
        i: '0',
    };
    let ins_cap: Captures;
    let re_on = Regex::new(r"turn on ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)").unwrap();
    let re_off = Regex::new(r"turn off ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)").unwrap();
    let re_toggle = Regex::new(r"toggle ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)").unwrap();
    if let Some(caps) = re_on.captures(str.as_str()) {
        ins_cap = caps;
        ins.i = 's';
    } else if let Some(caps) = re_off.captures(str.as_str()) {
        ins_cap = caps;
        ins.i = 'r';
    } else if let Some(caps) = re_toggle.captures(str.as_str()) {
        ins_cap = caps;
        ins.i = 't';
    } else {
        // dbg!(str);
        panic!();
    };
    // dbg!(&ins_cap);
    ins.x1 = ins_cap[1].parse::<u16>().unwrap();
    ins.y1 = ins_cap[2].parse::<u16>().unwrap();
    ins.x2 = ins_cap[3].parse::<u16>().unwrap();
    ins.y2 = ins_cap[4].parse::<u16>().unwrap();
    ins
}

fn main() {
    let str_list = read_lines("./src/input.txt");
    let mut lights: Vec<Vec<u32>> = vec![vec![0; 1000]; 1000];
    // dbg!(lights);
    for str in str_list {
        let ins: Instruction = parse_string(str);
        // dbg!(ins.x1, ins.y1, ins.x2, ins.y2, ins.i);
        for x in ins.x1..(ins.x2 + 1) {
            for y in ins.y1..(ins.y2 + 1) {
                match ins.i {
                    's' => {
                        lights[x as usize][y as usize] += 1;
                    }
                    'r' => {
                        if lights[x as usize][y as usize] > 0 {
                            lights[x as usize][y as usize] -= 1;
                        }
                    }
                    't' => {
                        lights[x as usize][y as usize] += 2;
                    }
                    _ => {
                        panic!()
                    }
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
    dbg!(num);
}

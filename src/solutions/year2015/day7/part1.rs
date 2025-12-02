use regex::{Captures, Regex};
use std::collections::HashMap;
use std::fs;
use crate::Runnable;

pub struct Solution;

#[derive(Debug)]
struct Instruction {
    opr1: String,
    opr2: String,
    op: String,
    out: String,
}

#[derive(Debug)]
struct Expression {
    opr1: u16,
    opr2: u16,
    op: String,
}

impl Solution {
    fn read_lines(filename: &str) -> Vec<String> {
        fs::read_to_string(filename)
            .unwrap()
            .lines()
            .map(String::from)
            .collect()
    }

    fn parse_string(s: String) -> Instruction {
        let mut ins = Instruction {
            opr1: String::new(),
            opr2: String::new(),
            op: String::new(),
            out: String::new(),
        };
        let re1 = Regex::new(r"([A-Za-z0-9]+) ([A-Za-z]+) ([A-Za-z0-9]+) -> ([A-Za-z]+)").unwrap();
        let re2 = Regex::new(r"([A-Za-z]+) ([A-Za-z0-9]+) -> ([A-Za-z]+)").unwrap();
        let re3 = Regex::new(r"([A-Za-z0-9]+) -> ([A-Za-z]+)").unwrap();
        if let Some(caps) = re1.captures(&s) {
            ins.opr1 = caps[1].to_string();
            ins.op = caps[2].to_string();
            ins.opr2 = caps[3].to_string();
            ins.out = caps[4].to_string();
        } else if let Some(caps) = re2.captures(&s) {
            ins.op = caps[1].to_string();
            ins.opr1 = caps[2].to_string();
            ins.out = caps[3].to_string();
        } else if let Some(caps) = re3.captures(&s) {
            ins.opr1 = caps[1].to_string();
            ins.out = caps[2].to_string();
        } else {
            panic!("Invalid instruction: {}", s);
        }
        ins
    }

    fn evaluate(exp: &Expression) -> u16 {
        match exp.op.as_str() {
            "AND" => exp.opr1 & exp.opr2,
            "OR" => exp.opr1 | exp.opr2,
            "RSHIFT" => exp.opr1 >> exp.opr2,
            "LSHIFT" => exp.opr1 << exp.opr2,
            "NOT" => !exp.opr1,
            "" => exp.opr1,
            _ => 0,
        }
    }

    fn can_evaluate(out_hash_map: &HashMap<&str, u16>, ins: &Instruction) -> (Expression, bool) {
        let mut exp = Expression {
            opr1: 0,
            opr2: 0,
            op: String::new(),
        };

        let re_num = Regex::new(r"[0-9]+").unwrap();

        if ["AND", "OR", "RSHIFT", "LSHIFT"].contains(&ins.op.as_str()) {
            exp.op = ins.op.clone();

            let mut opr1_ok = false;
            if out_hash_map.contains_key(ins.opr1.as_str()) {
                exp.opr1 = *out_hash_map.get(ins.opr1.as_str()).unwrap();
                opr1_ok = true;
            } else if re_num.captures(&ins.opr1).is_some() {
                exp.opr1 = ins.opr1.parse().unwrap();
                opr1_ok = true;
            }

            let mut opr2_ok = false;
            if out_hash_map.contains_key(ins.opr2.as_str()) {
                exp.opr2 = *out_hash_map.get(ins.opr2.as_str()).unwrap();
                opr2_ok = true;
            } else if re_num.captures(&ins.opr2).is_some() {
                exp.opr2 = ins.opr2.parse().unwrap();
                opr2_ok = true;
            }

            if opr1_ok && opr2_ok {
                return (exp, true);
            }
        } else if ins.op == "NOT" || ins.op == "" {
            exp.op = ins.op.clone();

            let mut opr1_ok = false;
            if out_hash_map.contains_key(ins.opr1.as_str()) {
                exp.opr1 = *out_hash_map.get(ins.opr1.as_str()).unwrap();
                opr1_ok = true;
            } else if re_num.captures(&ins.opr1).is_some() {
                exp.opr1 = ins.opr1.parse().unwrap();
                opr1_ok = true;
            }

            if opr1_ok {
                return (exp, true);
            }
        }
        (exp, false)
    }

    pub fn solve(input: &str) -> u16 {
        let str_list: Vec<String> = input.lines().map(String::from).collect();
        let mut ins_list: Vec<Instruction> = vec![];
        for s in str_list {
            ins_list.push(Self::parse_string(s));
        }

        let mut out_hash_map: HashMap<&str, u16> = HashMap::new();
        let mut uneval_exp_num = ins_list.len();

        while uneval_exp_num > 0 {
            for ins in &ins_list {
                if out_hash_map.contains_key(ins.out.as_str()) {
                    continue;
                }
                let (exp, is_exp) = Self::can_evaluate(&out_hash_map, ins);
                if is_exp {
                    let out_val = Self::evaluate(&exp);
                    out_hash_map.insert(ins.out.as_str(), out_val);
                    uneval_exp_num -= 1;
                }
            }
        }
        *out_hash_map.get("a").unwrap_or(&0)
    }
}

impl Runnable for Solution {
    fn run() {
        let contents = fs::read_to_string("src/2015/day7/input.txt").expect("Failed to read input");
        let result = Solution::solve(&contents);
        println!("{:?}", result);
    }
}

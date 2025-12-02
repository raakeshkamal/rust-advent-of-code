use regex::{Captures, Regex};
use std::collections::HashMap;
use std::{fs, vec};
use rust_advent_of_code::Runnable;

// Example: To run an active solution, uncomment and adjust:
// use rust_advent_of_code::active::year2015::dayX::part1::Solution;

fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

struct Instruction {
    opr1: String,
    opr2: String,
    op: String,
    out: String,
}

struct Expression {
    opr1: u16,
    opr2: u16,
    op: String,
}

fn parse_string(str: String) -> Instruction {
    let mut ins: Instruction = Instruction {
        opr1: String::new(),
        opr2: String::new(),
        op: String::new(),
        out: String::new(),
    };
    let ins_cap: Captures;
    let re1 = Regex::new(r"([A-Za-z0-9]+) ([A-Za-z]+) ([A-Za-z0-9]+) -> ([A-Za-z]+)").unwrap();
    let re2 = Regex::new(r"([A-Za-z]+) ([A-Za-z0-9]+) -> ([A-Za-z]+)").unwrap();
    let re3 = Regex::new(r"([A-Za-z0-9]+) -> ([A-Za-z]+)").unwrap();
    if let Some(caps) = re1.captures(str.as_str()) {
        ins_cap = caps;
        ins.opr1 = ins_cap[1].to_string();
        ins.op = ins_cap[2].to_string();
        ins.opr2 = ins_cap[3].to_string();
        ins.out = ins_cap[4].to_string();
    } else if let Some(caps) = re2.captures(str.as_str()) {
        ins_cap = caps;
        ins.op = ins_cap[1].to_string();
        ins.opr1 = ins_cap[2].to_string();
        ins.out = ins_cap[3].to_string();
    } else if let Some(caps) = re3.captures(str.as_str()) {
        ins_cap = caps;
        ins.opr1 = ins_cap[1].to_string();
        ins.out = ins_cap[2].to_string();
    } else {
        // dbg!(str);
        panic!();
    };
    // dbg!(&ins_cap);
    ins
}

fn evaluate(exp: &Expression) -> u16 {
    let mut out: u16 = 0;
    if exp.op == "AND" {
        out = exp.opr1 & exp.opr2;
    } else if exp.op == "OR" {
        out = exp.opr1 | exp.opr2;
    } else if exp.op == "RSHIFT" {
        out = exp.opr1 >> exp.opr2;
    } else if exp.op == "LSHIFT" {
        out = exp.opr1 << exp.opr2;
    } else if exp.op == "NOT" {
        out = !exp.opr1;
    } else if exp.op == "" {
        out = exp.opr1;
    }
    out
}

fn can_evaluate(out_hash_map_ref: &HashMap<&str, u16>, ins: &Instruction) -> (Expression, bool) {
    let mut exp: Expression = Expression {
        opr1: 0,
        opr2: 0,
        op: String::new(),
    };

    if ins.op == "AND" || ins.op == "OR" || ins.op == "RSHIFT" || ins.op == "LSHIFT" {
        //2 operands
        //can be numbers
        //can be found in hashmap
        let re = Regex::new(r"([0-9]+)").unwrap();
        let mut opr1_ok = false;
        let mut opr2_ok = false;
        exp.op = ins.op.clone();

        if out_hash_map_ref.contains_key(ins.opr1.as_str()) {
            exp.opr1 = out_hash_map_ref[ins.opr1.as_str()];
            opr1_ok = true;
        } else if let Some(_caps) = re.captures(ins.opr1.as_str()) {
            exp.opr1 = ins.opr1.parse::<u16>().unwrap();
            opr1_ok = true;
        }

        if out_hash_map_ref.contains_key(ins.opr2.as_str()) {
            exp.opr2 = out_hash_map_ref[ins.opr2.as_str()];
            opr2_ok = true;
        } else if let Some(_caps) = re.captures(ins.opr2.as_str()) {
            exp.opr2 = ins.opr2.parse::<u16>().unwrap();
            opr2_ok = true;
        }

        if opr1_ok && opr2_ok {
            return (exp, true);
        }
    } else if ins.op == "NOT" || ins.op == "" {
        //1 operand
        //can be number
        //can be found in hashmap
        let re = Regex::new(r"([0-9]+)").unwrap();
        let mut opr1_ok = false;
        exp.op = ins.op.clone();

        if out_hash_map_ref.contains_key(ins.opr1.as_str()) {
            exp.opr1 = out_hash_map_ref[ins.opr1.as_str()];
            opr1_ok = true;
        } else if let Some(_caps) = re.captures(ins.opr1.as_str()) {
            exp.opr1 = ins.opr1.parse::<u16>().unwrap();
            opr1_ok = true;
        }

        if opr1_ok {
            return (exp, true);
        }
    }
    (exp, false)
}

fn main() {
    // Solution::run();
    println!("Setup complete. Edit main.rs to import and run your active solution.");
}

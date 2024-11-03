use std::fs;
use std::collections::HashMap;

fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
    .unwrap()
    .lines()
    .map(String::from)
    .collect()
}

fn main() {
    let str_list = read_lines("./src/input.txt");
    let mut nice:u32 = 0;
    for str in str_list{
        // let mut is_nice = true;  
        let mut reps:u32 = 0;
        let byte_str = str.as_bytes();
        let str_len = str.len();
        let mut str_pos_hash:HashMap<&str, usize> = HashMap::new();
        let mut dual_reps:u32 = 0;
        dbg!(str.clone());
        for i in 0..str_len-2 { 
            if byte_str[i] == byte_str[i+2]{
                dbg!("reps");
                reps += 1;
            }
        }
        for i in 0..str_len-1 {
            let substr = &str[i..i+2];
            dbg!(substr);
            if str_pos_hash.contains_key(substr) {
                let temp = str_pos_hash[substr];
                if (i-temp) > 1 {
                    // dbg!(str.clone());
                    dbg!("dual");
                    dual_reps += 1;
                }
                str_pos_hash.entry(substr).and_modify(|f| *f=i);
            } else {
                str_pos_hash.insert(substr, i);
            }
        }
        if dual_reps > 0 {
            dbg!(str_pos_hash);
        }
        if reps > 0 && dual_reps > 0{
            dbg!("nice");
            nice +=1;
        }
    }
    dbg!(nice);
}

use std::fs;

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
        let mut is_nice = true;
        let mut reps:u32 = 0;
        let mut vowels:u32 = 0;
        let byte_str = str.as_bytes();
        let str_len = str.len();
        dbg!(str.clone());
        for i in 0..str_len-1 {
            let substr = &str[i..i+2];
            if substr == "ab" || substr == "cd" || substr == "pq" || substr == "xy" {
                dbg!("fail");
                is_nice = false;
                break;
            }
            if byte_str[i] == byte_str[i+1]{
                dbg!("reps");
                reps += 1;
            }
            let temp = byte_str[i] as char;
            if temp == 'a' || temp == 'e' || temp == 'i' || temp == 'o' || temp == 'u'{
                dbg!("vowels");
                vowels += 1;
            }
        }
        let temp = byte_str[str_len-1] as char;
        if temp == 'a' || temp == 'e' || temp == 'i' || temp == 'o' || temp == 'u'{
            dbg!("vowels");
            vowels += 1;
        }
        if is_nice && reps > 0 && vowels > 2{
            dbg!("nice");
            nice +=1;
        }
    }
    dbg!(nice);
}

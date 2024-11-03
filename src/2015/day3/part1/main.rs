use std::fs;
use std::collections::HashMap;

// fn read_lines(filename: &str) -> Vec<String> {
//     fs::read_to_string(filename)
//     .unwrap()
//     .lines()
//     .map(String::from)
//     .collect()
// }


fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Failed to read ./src/input.txt");
    // dbg!(contents);
    let mut gifts:HashMap<(i32,i32),u32> = HashMap::new();
    let mut x:i32 = 0;
    let mut y:i32 = 0;
    gifts.insert((x,y),1);
    let byte_str = contents.as_bytes();
    for i in byte_str{
        let temp = *i as char;
        if temp == '>' {
            x += 1;
        } else if temp == '^' {
            y += 1;
        } else if temp == 'v' {
            y -= 1;
        } else if temp == '<' {
            x -= 1;
        }
        let location = (x,y);
        gifts.entry(location).and_modify(|ctr| *ctr+=1).or_insert(1);
    }
    dbg!(gifts.keys().len());
    // let box_list: Vec<String> = read_lines("./src/input.txt");
    // let mut area:u64 = 0;
    // let mut ribbon:u64 = 0;
    // for str in box_list{
    //     let mut dimensions: Vec<u64> = str.split("x").map(|x| x.parse::<u64>().unwrap()).collect();
    //     dimensions.sort();
    //     ribbon += 2*(dimensions[0] + dimensions[1]);
    //     ribbon += dimensions[0] * dimensions[1] * dimensions[2];
    //     area += 3 * dimensions[0] * dimensions[1];
    //     area += 2 * dimensions[1] * dimensions[2];
    //     area += 2 * dimensions[0] * dimensions[2];
    // }
    // dbg!(area);
    // dbg!(ribbon);
}

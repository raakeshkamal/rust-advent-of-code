use std::fs;

fn main() {
    let contents = fs::read_to_string("./src/input.txt").expect("Failed to read ./src/input.txt");
    println!("With text:\n{contents}");
    let byte_str = contents.as_bytes();
    let mut floor = 0;
    let mut pos = 0;
    for i in byte_str{
        if *i as char == '(' {
            floor += 1;
        } else if *i as char == ')' {
            floor -= 1;
        }
        pos += 1;
        if floor < 0 {
            break;
        }
    }
    println!("{:?}",pos);
}

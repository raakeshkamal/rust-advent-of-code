use std::fs;

fn read_lines(filename: &str) -> Vec<String> {
    fs::read_to_string(filename)
    .unwrap()
    .lines()
    .map(String::from)
    .collect()
}

fn main() {
    let box_list: Vec<String> = read_lines("./src/input.txt");
    let mut area:u64 = 0;
    let mut ribbon:u64 = 0;
    for str in box_list{
        let mut dimensions: Vec<u64> = str.split("x").map(|x| x.parse::<u64>().unwrap()).collect();
        dimensions.sort();
        ribbon += 2*(dimensions[0] + dimensions[1]);
        ribbon += dimensions[0] * dimensions[1] * dimensions[2];
        area += 3 * dimensions[0] * dimensions[1];
        area += 2 * dimensions[1] * dimensions[2];
        area += 2 * dimensions[0] * dimensions[2];
    }
    dbg!(area);
    dbg!(ribbon);
}

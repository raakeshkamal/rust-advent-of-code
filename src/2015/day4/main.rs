use md5;

fn main() {
    let input = b"yzbqklnj";
    let mut i:u32 = 0;
    loop {
        let temp = i.to_string();
        let test = temp.as_bytes();
        let res = [input,test].concat();
        let out = format!("{:X}",md5::compute(res));
        if &out[..6] == "000000" {
            break;
        }
        i += 1;
    }
    dbg!(i);
}

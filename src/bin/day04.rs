fn main() {
    let input = "bgvyzdsv";

    for i in 1.. {
        let s = input.to_string() + &i.to_string();
        let digest = format!("{:x}", md5::compute(s));
        if digest.starts_with("00000") {
            println!("Part one: the answer is {}", i);
            break;
        }
    }

    for i in 1.. {
        let s = input.to_string() + &i.to_string();
        let digest = format!("{:x}", md5::compute(s));
        if digest.starts_with("000000") {
            println!("Part two: the answer is {}", i);
            break;
        }
    }
}

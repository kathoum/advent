fn main() {
    let mut pwd = *b"cqjxjnds";

    print!("The password after {} is ", std::str::from_utf8(&pwd).unwrap());
    next_valid(&mut pwd);
    println!("{}", std::str::from_utf8(&pwd).unwrap());
    next_valid(&mut pwd);
    println!("The next one is {}", std::str::from_utf8(&pwd).unwrap());
}

fn increment(pwd: &mut [u8; 8]) {
    for c in pwd.iter_mut().rev() {
        if *c == b'z' {
            *c = b'a';
        } else {
            *c += 1;
            break;
        }
    }
}

fn check(pwd: &[u8; 8]) -> bool {
    if pwd.windows(3).find(|wnd| wnd[1] == wnd[0] + 1 && wnd[2] == wnd[1] + 1).is_none() {
        false
    } else if pwd.contains(&b'i') || pwd.contains(&b'o') || pwd.contains(&b'l') {
        false
    } else {
        let i = pwd.windows(2).position(|wnd| wnd[0] == wnd[1]);
        let j = pwd.windows(2).rev().position(|wnd| wnd[0] == wnd[1]);
        match (i, j) {
            (Some(a), Some(b)) => a + b <= pwd.len() - 4,
            _ => false
        }
    }
}

fn next_valid(pwd: &mut [u8; 8]) {
    loop {
        increment(pwd);
        if check(pwd) {
            break
        }
    }
}

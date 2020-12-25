fn main() {
    let modulus = 20201227;
    let base = 7;
    let card_pk = 18356117;
    let door_pk = 5909654;

    println!("Part One");
    let card_exp = log(modulus, base, card_pk);
    let door_exp = log(modulus, base, door_pk);
    println!("Loop sizes = {} and {}", card_exp, door_exp);
    println!("Key = {} = {}", exp(modulus, card_pk, door_exp), exp(modulus, door_pk, card_exp));
}

fn log(modulus: u64, base: u64, n: u64) -> u64 {
    let (mut exp, mut m) = (1, base);
    while m != n {
        m = (m * base) % modulus;
        exp += 1;
    }
    exp
}

fn exp(modulus: u64, base: u64, exp: u64) -> u64 {
    let mut n = 1;
    for _ in 0..exp {
        n = (n * base) % modulus;
    }
    n
}

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut cache = HashMap::new();

    let (complexity1, complexity2) = BufReader::new(File::open("input/day21.txt").unwrap())
        .lines()
        .fold((0, 0), |(c1, c2), line| {
            let code = line.unwrap();
            let len1 = recursive_length(&mut cache, &code, 2);
            let len2 = recursive_length(&mut cache, &code, 25);
            let val = code
                .trim_start_matches('0')
                .trim_end_matches('A')
                .parse::<usize>()
                .unwrap();
            (c1 + len1 * val, c2 + len2 * val)
        });
    println!("Day 21 part one: {complexity1}");
    println!("Day 21 part two: {complexity2}");
}

fn recursive_length(cache: &mut HashMap<(String, i32), usize>, code: &str, depth: i32) -> usize {
    code.split_inclusive('A')
        .map(|chunk| recursive_chunk_length(cache, chunk, depth))
        .sum()
}

fn recursive_chunk_length(
    cache: &mut HashMap<(String, i32), usize>,
    chunk: &str,
    depth: i32,
) -> usize {
    assert!(chunk.ends_with('A'));
    assert_eq!(chunk.chars().filter(|c| *c == 'A').count(), 1);
    if depth < 0 {
        chunk.len()
    } else if let Some(n) = cache.get(&(chunk.to_owned(), depth)) {
        *n
    } else {
        let n = expand_sequences(chunk)
            .into_iter()
            .map(|seq| recursive_length(cache, &seq, depth - 1))
            .min()
            .unwrap();
        cache.insert((chunk.to_owned(), depth), n);
        n
    }
}

fn expand_sequences(code: &str) -> Vec<String> {
    let mut pos = b'A';
    let mut seq = vec![String::new()];
    for next in code.bytes() {
        let additional = get_sequences(pos, next);
        pos = next;
        let mut newseq = vec![];
        for seq in seq {
            for &add in additional {
                newseq.push([&seq, add, "A"].concat());
            }
        }
        seq = newseq;
    }
    seq
}

fn get_sequences(from: u8, to: u8) -> &'static [&'static str] {
    sequences(get_move(from, to))
}

fn get_move(from: u8, to: u8) -> Move {
    match (from, to) {
        (b'0', b'0') => Move::N,
        (b'0', b'1') => Move::UL1,
        (b'0', b'2') => Move::U,
        (b'0', b'3') => Move::UR,
        (b'0', b'4') => Move::UUL1,
        (b'0', b'5') => Move::UU,
        (b'0', b'6') => Move::UUR,
        (b'0', b'7') => Move::UUUL1,
        (b'0', b'8') => Move::UUU,
        (b'0', b'9') => Move::UUUR,
        (b'0', b'A') => Move::R,

        (b'1', b'0') => Move::DR1,
        (b'1', b'1') => Move::N,
        (b'1', b'2') => Move::R,
        (b'1', b'3') => Move::RR,
        (b'1', b'4') => Move::U,
        (b'1', b'5') => Move::UR,
        (b'1', b'6') => Move::URR,
        (b'1', b'7') => Move::UU,
        (b'1', b'8') => Move::UUR,
        (b'1', b'9') => Move::UURR,
        (b'1', b'A') => Move::DRR1,

        (b'2', b'0') => Move::D,
        (b'2', b'1') => Move::L,
        (b'2', b'2') => Move::N,
        (b'2', b'3') => Move::R,
        (b'2', b'4') => Move::UL,
        (b'2', b'5') => Move::U,
        (b'2', b'6') => Move::UR,
        (b'2', b'7') => Move::UUL,
        (b'2', b'8') => Move::UU,
        (b'2', b'9') => Move::UUR,
        (b'2', b'A') => Move::DR,

        (b'3', b'0') => Move::DL,
        (b'3', b'1') => Move::LL,
        (b'3', b'2') => Move::L,
        (b'3', b'3') => Move::N,
        (b'3', b'4') => Move::ULL,
        (b'3', b'5') => Move::UL,
        (b'3', b'6') => Move::U,
        (b'3', b'7') => Move::UULL,
        (b'3', b'8') => Move::UUL,
        (b'3', b'9') => Move::UU,
        (b'3', b'A') => Move::D,

        (b'4', b'0') => Move::DDR1,
        (b'4', b'1') => Move::D,
        (b'4', b'2') => Move::DR,
        (b'4', b'3') => Move::DRR,
        (b'4', b'4') => Move::N,
        (b'4', b'5') => Move::R,
        (b'4', b'6') => Move::RR,
        (b'4', b'7') => Move::U,
        (b'4', b'8') => Move::UR,
        (b'4', b'9') => Move::URR,
        (b'4', b'A') => Move::DDRR1,

        (b'5', b'0') => Move::DD,
        (b'5', b'1') => Move::DL,
        (b'5', b'2') => Move::D,
        (b'5', b'3') => Move::DR,
        (b'5', b'4') => Move::L,
        (b'5', b'5') => Move::N,
        (b'5', b'6') => Move::R,
        (b'5', b'7') => Move::UL,
        (b'5', b'8') => Move::U,
        (b'5', b'9') => Move::UR,
        (b'5', b'A') => Move::DDR,

        (b'6', b'0') => Move::DDL,
        (b'6', b'1') => Move::DLL,
        (b'6', b'2') => Move::DL,
        (b'6', b'3') => Move::D,
        (b'6', b'4') => Move::LL,
        (b'6', b'5') => Move::L,
        (b'6', b'6') => Move::N,
        (b'6', b'7') => Move::ULL,
        (b'6', b'8') => Move::UL,
        (b'6', b'9') => Move::U,
        (b'6', b'A') => Move::DD,

        (b'7', b'0') => Move::DDDR1,
        (b'7', b'1') => Move::DD,
        (b'7', b'2') => Move::DDR,
        (b'7', b'3') => Move::DDRR,
        (b'7', b'4') => Move::D,
        (b'7', b'5') => Move::DR,
        (b'7', b'6') => Move::DRR,
        (b'7', b'7') => Move::N,
        (b'7', b'8') => Move::R,
        (b'7', b'9') => Move::RR,
        (b'7', b'A') => Move::DDDRR1,

        (b'8', b'0') => Move::DDD,
        (b'8', b'1') => Move::DDL,
        (b'8', b'2') => Move::DD,
        (b'8', b'3') => Move::DDR,
        (b'8', b'4') => Move::DL,
        (b'8', b'5') => Move::D,
        (b'8', b'6') => Move::DR,
        (b'8', b'7') => Move::L,
        (b'8', b'8') => Move::N,
        (b'8', b'9') => Move::R,
        (b'8', b'A') => Move::DDDR,

        (b'9', b'0') => Move::DDDL,
        (b'9', b'1') => Move::DDLL,
        (b'9', b'2') => Move::DDL,
        (b'9', b'3') => Move::DD,
        (b'9', b'4') => Move::DLL,
        (b'9', b'5') => Move::DL,
        (b'9', b'6') => Move::D,
        (b'9', b'7') => Move::LL,
        (b'9', b'8') => Move::L,
        (b'9', b'9') => Move::N,
        (b'9', b'A') => Move::DDD,

        (b'A', b'0') => Move::L,
        (b'A', b'1') => Move::ULL1,
        (b'A', b'2') => Move::UL,
        (b'A', b'3') => Move::U,
        (b'A', b'4') => Move::UULL1,
        (b'A', b'5') => Move::UUL,
        (b'A', b'6') => Move::UU,
        (b'A', b'7') => Move::UUULL1,
        (b'A', b'8') => Move::UUUL,
        (b'A', b'9') => Move::UUU,
        (b'A', b'A') => Move::N,
        (b'A', b'<') => Move::DLL1,
        (b'A', b'>') => Move::D,
        (b'A', b'^') => Move::L,
        (b'A', b'v') => Move::DL,

        (b'<', b'<') => Move::N,
        (b'<', b'>') => Move::RR,
        (b'<', b'^') => Move::UR1,
        (b'<', b'v') => Move::R,
        (b'<', b'A') => Move::URR1,

        (b'>', b'<') => Move::LL,
        (b'>', b'>') => Move::N,
        (b'>', b'^') => Move::UL,
        (b'>', b'v') => Move::L,
        (b'>', b'A') => Move::U,

        (b'^', b'<') => Move::DL1,
        (b'^', b'>') => Move::DR,
        (b'^', b'^') => Move::N,
        (b'^', b'v') => Move::D,
        (b'^', b'A') => Move::R,

        (b'v', b'<') => Move::L,
        (b'v', b'>') => Move::R,
        (b'v', b'^') => Move::U,
        (b'v', b'v') => Move::N,
        (b'v', b'A') => Move::UR,

        _ => panic!("Unexpected move {}-{}", from as char, to as char),
    }
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy)]
enum Move {
    N,
    U,
    D,
    R,
    L,
    UU,
    DD,
    RR,
    LL,
    UR,
    UR1,
    UL,
    UL1,
    DR,
    DR1,
    DL,
    DL1,
    UUU,
    UUR,
    UUL,
    UUL1,
    URR,
    URR1,
    ULL,
    ULL1,
    DRR,
    DRR1,
    DLL,
    DLL1,
    DDR,
    DDR1,
    DDL,
    DDD,
    UUUR,
    UUUL,
    UUUL1,
    UURR,
    UULL,
    UULL1,
    DDRR,
    DDRR1,
    DDLL,
    DDDR,
    DDDR1,
    DDDL,
    DDDRR1,
    UUULL1,
}

fn sequences(m: Move) -> &'static [&'static str] {
    match m {
        Move::N => &[""],
        Move::U => &["^"],
        Move::D => &["v"],
        Move::R => &[">"],
        Move::L => &["<"],
        Move::UU => &["^^"],
        Move::DD => &["vv"],
        Move::RR => &[">>"],
        Move::LL => &["<<"],
        Move::UR => &["^>", ">^"],
        Move::UR1 => &[">^"],
        Move::UL => &["^<", "<^"],
        Move::UL1 => &["^<"],
        Move::DR => &["v>", ">v"],
        Move::DR1 => &[">v"],
        Move::DL => &["v<", "<v"],
        Move::DL1 => &["v<"],
        Move::UUU => &["^^^"],
        Move::UUR => &["^^>", "^>^", ">^^"],
        Move::UUL => &["^^<", "^<^", "<^^"],
        Move::UUL1 => &["^^<", "^<^"],
        Move::URR => &["^>>", ">^>", ">>^"],
        Move::URR1 => &[">^>", ">>^"],
        Move::ULL => &["^<<", "<^<", "<<^"],
        Move::ULL1 => &["^<<", "<^<"],
        Move::DRR => &["v>>", ">v>", ">>v"],
        Move::DRR1 => &[">v>", ">>v"],
        Move::DLL => &["v<<", "<v<", "<<v"],
        Move::DLL1 => &["v<<", "<v<"],
        Move::DDR => &["vv>", "v>v", ">vv"],
        Move::DDR1 => &["v>v", ">vv"],
        Move::DDL => &["vv<", "v<v", "<vv"],
        Move::DDD => &["vvv"],
        Move::UUUR => &["^^^>", "^^>^", "^>^^", ">^^^"],
        Move::UUUL => &["^^^<", "^^<^", "^<^^", "<^^^"],
        Move::UUUL1 => &["^^^<", "^^<^", "^<^^"],
        Move::UURR => &["^^>>", "^>^>", "^>>^", ">^^>", ">^>^", ">>^^"],
        Move::UULL => &["^^<<", "^<^<", "^<<^", "<^^<", "<^<^", "<<^^"],
        Move::UULL1 => &["^^<<", "^<^<", "^<<^", "<^^<", "<^<^"],
        Move::DDRR => &["vv>>", "v>v>", "v>>v", ">vv>", ">v>v", ">>vv"],
        Move::DDRR1 => &["v>v>", "v>>v", ">vv>", ">v>v", ">>vv"],
        Move::DDLL => &["vv<<", "v<v<", "v<<v", "<vv<", "<v<v", "<<vv"],
        Move::DDDR => &["vvv>", "vv>v", "v>vv", ">vvv"],
        Move::DDDR1 => &["vv>v", "v>vv", ">vvv"],
        Move::DDDL => &["vvv<", "vv<v", "v<vv", "<vvv"],
        Move::DDDRR1 => &[
            "vv>v>", "vv>>v", "v>vv>", "v>v>v", "v>>vv", ">vvv>", ">vv>v", ">v>vv", ">>vvv",
        ],
        Move::UUULL1 => &[
            "^^^<<", "^^<^<", "^^<<^", "^<^^<", "^<^<^", "^<<^^", "<^^^<", "<^^<^", "<^<^^",
        ],
    }
}

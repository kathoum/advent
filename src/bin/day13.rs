use std::fs::File;
use std::io::{BufRead,BufReader};
use std::cmp::Ordering;
use std::str::FromStr;

fn main() {
    let reader = BufReader::new(File::open("input/day13.txt").unwrap());

    let mut lines = reader.lines();
    let mut counter = 1;
    let mut score = 0;
    let mut all_packets = Vec::new();
    loop {
        let first = lines.next().unwrap().unwrap().parse::<Packet>().unwrap();
        let second = lines.next().unwrap().unwrap().parse::<Packet>().unwrap();

        if first <= second {
            score += counter;
        }
        counter += 1;

        all_packets.push(first);
        all_packets.push(second);

        match lines.next() {
            None => break,
            Some(l) => assert!(l.unwrap().is_empty())
        }
    }

    println!("The indices of the in-order pairs sum to {score}");

    let p2 = "[[2]]".parse::<Packet>().unwrap();
    let p6 = "[[6]]".parse::<Packet>().unwrap();
    all_packets.push(p2.clone());
    all_packets.push(p6.clone());
    all_packets.sort();
    let i2 = all_packets.iter().position(|p| p == &p2).unwrap() + 1;
    let i6 = all_packets.iter().position(|p| p == &p6).unwrap() + 1;

    println!("The decoder key is {}", i2 * i6);
}

#[derive(Clone, Eq)]
enum Packet {
    Value(i64),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = serde_json::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: serde_json::Value = serde_json::from_str(s)?;
        Ok(v.into())
    }
}

impl From<serde_json::Value> for Packet {
    fn from(v: serde_json::Value) -> Self {
        match v {
            serde_json::Value::Number(n) => Packet::Value(n.as_i64().unwrap()),
            serde_json::Value::Array(a) => Packet::List(a.into_iter().map(Into::into).collect()),
            _ => panic!()
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Value(i), Packet::Value(j)) => i.cmp(j),
            (Packet::List(ref v), Packet::List(ref w)) => v.cmp(w),
            (Packet::Value(i), Packet::List(ref w)) => std::slice::from_ref(&Packet::Value(*i)).cmp(&w[..]),
            (Packet::List(ref v), Packet::Value(j)) => v[..].cmp(std::slice::from_ref(&Packet::Value(*j))),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

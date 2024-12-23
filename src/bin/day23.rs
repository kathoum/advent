use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut links: HashMap<Node, Vec<Node>> = HashMap::new();
    for line in BufReader::new(File::open("input/day23.txt").unwrap()).lines() {
        match line.unwrap().as_bytes() {
            &[a1, a2, b'-', b1, b2] => {
                let a = Node::from([a1, a2]);
                let b = Node::from([b1, b2]);
                assert_ne!(a, b);
                let (a, b) = (a.min(b), a.max(b));
                links.entry(a).or_default().push(b);
                links.entry(b).or_default();
            }
            _ => panic!(),
        }
    }
    for v in links.values_mut() {
        v.sort();
    }
    let links = links;

    let mut triples = 0;
    for (&a, av) in &links {
        for (ib, &b) in av.iter().enumerate() {
            for &c in &av[ib + 1..] {
                if links[&b].contains(&c)
                    && (a.initial() == b't' || b.initial() == b't' || c.initial() == b't')
                {
                    triples += 1;
                }
            }
        }
    }
    println!("Day 23 part one: {triples}");

    let largest = links
        .keys()
        .map(|&a| largest_party(&links, &[a]))
        .max_by_key(Vec::len)
        .unwrap();
    let name = largest
        .into_iter()
        .map(Node::name)
        .collect::<Vec<String>>()
        .join(",");
    println!("Day 23 part two: {name}");
}

fn largest_party(links: &HashMap<Node, Vec<Node>>, partial: &[Node]) -> Vec<Node> {
    let (first, last) = partial.split_first().unwrap();
    let mut friends = links[first].clone();
    for second in last {
        intersect(&mut friends, &links[second]);
    }
    friends
        .into_iter()
        .map(|friend| {
            let v = [partial, &[friend]].concat();
            largest_party(links, &v)
        })
        .max_by_key(Vec::len)
        .unwrap_or_else(|| partial.to_vec())
}

fn intersect<T: Ord>(v: &mut Vec<T>, w: &[T]) {
    let mut i = 0;
    v.retain(|x| {
        while let Some(y) = w.get(i) {
            match x.cmp(y) {
                Ordering::Equal => return true,
                Ordering::Less => return false,
                Ordering::Greater => i += 1,
            }
        }
        false
    });
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
struct Node(u16);

impl Node {
    fn from(name: [u8; 2]) -> Self {
        Self(u16::from_be_bytes([name[0], name[1]]))
    }
    fn name(self) -> String {
        String::from_utf8(self.0.to_be_bytes().to_vec()).unwrap()
    }
    fn initial(self) -> u8 {
        (self.0 >> 8) as u8
    }
}

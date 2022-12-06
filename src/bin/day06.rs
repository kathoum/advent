fn main() {
    let bytes = std::fs::read("input/day06.txt").unwrap();

    let first_marker = first_distinct(&bytes, 4) + 1;
    println!("The first marker is after {} characters", first_marker);

    let first_message = first_distinct(&bytes, 14) + 1;
    println!("The first message is after {} characters", first_message);
}

fn first_distinct(msg: &[u8], len: usize) -> usize {
    let mut counter = Counter::new();
    for &byte in &msg[..len-1] {
        counter.push(byte);
    }
    for i in (len-1)..msg.len() {
        counter.push(msg[i]);
        if counter.count_distinct() == len {
            return i;
        }
        counter.pop(msg[i-(len-1)]);
    }
    0
}

struct Counter {
    data: [usize; 256],
    nonzero: usize,
}

impl Counter {
    pub fn new() -> Self { Counter { data: [0; 256], nonzero: 0 } }

    pub fn push(&mut self, byte: u8) {
        let p = &mut self.data[usize::from(byte)];
        if *p == 0 { self.nonzero += 1; }
        *p += 1;
    }

    pub fn pop(&mut self, byte: u8) {
        let p = &mut self.data[usize::from(byte)];
        *p -= 1;
        if *p == 0 { self.nonzero -= 1; }
    }

    pub fn count_distinct(&self) -> usize {
        self.nonzero
    }
}

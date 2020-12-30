fn main() {
    let input = &[9, 6, 2, 7, 1, 3, 8, 5, 4];
    assert_eq!(input.len() as u32, *input.iter().max().unwrap());

    println!("Part One");
    let mut data = input.to_vec();
    for _ in 0..100 {
        data = step(&data[..]);
    }
    println!("The labels after 1 are {}",
        data.iter().cloned()
        .cycle().skip_while(|&x| x != 1).skip(1).take_while(|&x| x != 1)
        .map(|i| std::char::from_digit(i as u32, 10).unwrap())
        .collect::<String>()
    );
    
    let mut deck = Deck::from(input.iter().cloned());
    for _ in 0..100 {
        deck.step();
    }
    println!("The labels after 1 are {}", deck.values()
        .cycle().skip_while(|&x| x != 1).skip(1).take_while(|&x| x != 1)
        .map(|i| std::char::from_digit(i as u32, 10).unwrap())
        .collect::<String>()
    );

    println!("Part Two");
    let mut data = input.to_vec();
    data.extend(input.len() as u32 + 1..=1_000_000);
    for _ in 0..10 {
        data = step(&data[..]);
    }
    let mut one = data.iter().skip_while(|&&x| x != 1).skip(1);
    let n1 = one.next().unwrap();
    let n2 = one.next().unwrap();
    println!("The stars are under {} and {} = {} (but I did only 10 iterations!)", n1, n2, n1 * n2);

    let mut deck = Deck::from(input.iter().cloned().chain((input.len() as u32 + 1)..=1_000_000));
    for _ in 0..10 {
        deck.step();
    }
    let mut one = deck.values().skip_while(|&x| x != 1).skip(1);
    let n1 = one.next().unwrap();
    let n2 = one.next().unwrap();
    println!("The stars are under {} and {} = {} (but I did only 10 iterations!)", n1, n2, n1 * n2);

    let mut deck = Deck::from(input.iter().cloned().chain((input.len() as u32 + 1)..=1_000_000));
    for _ in 0..10_000_000 {
        deck.step();
    }
    let mut one = deck.values().skip_while(|&x| x != 1).skip(1);
    let n1 = one.next().unwrap() as u64;
    let n2 = one.next().unwrap() as u64;
    println!("The stars are under {} and {} = {}", n1, n2, n1 * n2);
}

struct Deck {
    head: Node,
    tail: Node,
    lookup: Vec<Node>,
}

use std::rc::Rc;
use std::cell::Cell;
struct Item {
    next: Cell<Node>,
    value: u32,
}
type Node = Option<Rc<Item>>;

impl Deck {
    fn from(data: impl Iterator<Item = u32>) -> Self {
        let mut deck = Deck { head: None, tail: None, lookup: Vec::new() };
        for n in data {
            let node = Rc::new(Item { next: Cell::new(None), value: n });
            if deck.lookup.len() < n as usize + 1 {
                deck.lookup.resize_with(n as usize + 1, || None);
            }
            deck.lookup[n as usize] = Some(Rc::clone(&node));
            deck.insert_tail(node);
        }
        assert_eq!(deck.head.is_some(), deck.tail.is_some());
        assert!(deck.lookup[0].is_none());
        for i in deck.lookup[1..].iter() {
            assert!(i.is_some());
        }
        deck
    }

    fn step(&mut self) {
        let max = self.max();
        let current = self.pop_first().expect("Deck too small");
        let x1 = self.pop_first().expect("Deck too small");
        let x2 = self.pop_first().expect("Deck too small");
        let x3 = self.pop_first().expect("Deck too small");
        assert!(self.head.is_some(), "Deck too small");
        let mut dest = if current.value == 1 { max } else { current.value - 1 };
        while dest == x1.value || dest == x2.value || dest == x3.value {
            dest = if dest == 1 { max } else { dest - 1};
        }
        self.insert_after(dest, x3);
        self.insert_after(dest, x2);
        self.insert_after(dest, x1);
        self.insert_tail(current);
    }

    fn values(&self) -> impl Iterator<Item = u32> + Clone {
        DeckIterator { current: self.head.clone() }
    }

    fn max(&self) -> u32 {
        self.lookup.len() as u32 - 1
    }

    fn pop_first(&mut self) -> Node {
        let next = match &self.head {
            None => None,
            Some(p) => p.next.take(),
        };
        if next.is_none() {
            self.tail = None;
        }
        std::mem::replace(&mut self.head, next)
    }

    fn insert_after(&mut self, item: u32, x: Rc<Item>) {
        let node = self.lookup[item as usize].as_ref().expect("Item not found in deck");
        let next = node.next.take();
        if next.is_none() {
            assert!(Rc::ptr_eq(node, self.tail.as_ref().unwrap()));
            self.tail = Some(Rc::clone(&x));
        }
        let check = x.next.replace(next);
        assert!(check.is_none());
        node.next.set(Some(x));
    }

    fn insert_tail(&mut self, x: Rc<Item>) {
        match &self.tail {
            None => {
                assert!(self.head.is_none());
                let check = x.next.take();
                assert!(check.is_none());
                self.head = Some(Rc::clone(&x));
            }
            Some(p) => {
                assert!(self.head.is_some());
                let check = p.next.replace(Some(Rc::clone(&x)));
                assert!(check.is_none());
            }
        };
        self.tail = Some(x);
    }
}

#[derive(Clone)]
struct DeckIterator {
    current: Node
}

impl Iterator for DeckIterator {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        let (next, result) = match &self.current {
            None => (None, None),
            Some(p) => {
                let n = p.next.take();
                p.next.set(n.clone());
                (n, Some(p.value))
            }
        };
        self.current = next;
        result
    }
}

impl Drop for Deck {
    fn drop(&mut self) {
        while self.head.is_some() {
            self.pop_first();
        }
    }
}

fn step(data: &[u32]) -> Vec<u32> {
    let len = data.len();
    assert!(len >= 5);
    let current = data[0];
    let mut destination = current;
    while data[0..=3].contains(&destination) {
        destination = if destination == 1 { len as u32 } else { destination - 1 }
    }
    let dest = data.iter().position(|&x| x == destination).unwrap();
    assert!(dest > 3);
    let mut result = Vec::from(&data[4..=dest]);
    result.extend(&data[1..=3]);
    result.extend(&data[dest+1..]);
    result.extend(&data[0..1]);
    assert_eq!(result.len(), len);
    result
}

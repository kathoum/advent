fn main() {
    let input = include_str!("input13.txt");
    let (edges, names) = load(input);

    let max_value = permutations(&names).map(|order| total_value(&order, &edges, false)).max().unwrap();
    println!("Maximum happiness is {}", max_value);
    let max_value = permutations(&names).map(|order| total_value(&order, &edges, true)).max().unwrap();
    println!("Maximum happiness with me is {}", max_value);
}

struct Edge {
    a: String,
    b: String,
    value: i32,
}

fn load(input: &str) -> (Vec<Edge>, Vec<String>) {
    let edges: Vec<Edge> = input.lines().map(parse).collect();
    let mut names = Vec::new();
    for edge in edges.iter() {
        if !names.contains(&edge.a) { names.push(edge.a.clone()); }
        if !names.contains(&edge.b) { names.push(edge.b.clone()); }
    }
    names.sort();
    (edges, names)
}

fn parse(line: &str) -> Edge {
    let name1: String;
    let what: String;
    let amount: i32;
    let name2: String;
    text_io::scan!(line.bytes() => "{} would {} {} happiness units by sitting next to {}.", name1, what, amount, name2);
    let value = match what.as_str() {
        "gain" => amount,
        "lose" => - amount,
        _ => panic!()
    };
    Edge { a: name1, b: name2, value: value }
}

fn permutations<'a, T>(items: &'a [T]) -> Permutations<'a, T> {
    Permutations { items: items, counter: 0 }
}

struct Permutations<'a, T> {
    items: &'a [T],
    counter: usize,
}

impl<'a, T> Iterator for Permutations<'a, T> {
    type Item = Vec<&'a T>;
    fn next(&mut self) -> Option<Self::Item> {
        let max_counter = (1..=self.items.len()).product();
        if self.counter < max_counter {
            let mut temp: Vec<&'a T> = self.items.iter().collect();
            let mut result = Vec::with_capacity(temp.len());
            let mut counter = self.counter;
            while temp.len() > 0 {
                result.push(temp.remove(counter % temp.len()));
                counter /= temp.len() + 1;
            }
            self.counter += 1;
            Some(result)
        } else {
            None
        }
    }
}

fn total_value(order: &[&String], edges: &[Edge], with_me: bool) -> i32 {
    let value_for = |(a, b)| {
        let v1 = edges.iter().find(|e| &&e.a == a && &&e.b == b).unwrap().value;
        let v2 = edges.iter().find(|e| &&e.a == b && &&e.b == a).unwrap().value;
        v1 + v2
    };
    if with_me {
        order.iter().zip(order.iter().skip(1)).map(value_for).sum()
    } else {
        order.iter().zip(order.iter().cycle().skip(1)).map(value_for).sum()
    }
}

fn main() {
    let input = include_str!("input12.txt");

    let sum = numbers_in_string(input).sum::<i32>();
    println!("The sum of all numbers is {}", sum);

    let object = json::parse(input).unwrap();
    println!("The sum of all numbers is {} [json]", sum_of_numbers(&object));

    let clean_input = remove_objects_with_value(input, "\"red\"");
    let sum = numbers_in_string(&clean_input).sum::<i32>();
    println!("The sum without duplicates is {}", sum);

    println!("The sum without duplicates is {} [json]", sum_excluding_values(&object, "red"));
}

fn numbers_in_string<'a>(s: &'a str) -> Numbers<'a> {
    Numbers{s: s}
}

fn remove_objects_with_value(s: &str, key: &str) -> String {
    let mut s = String::from(s);
    while let Some(pos) = s.find(key) {
        let start = find_opening_parens(&s, pos).unwrap_or(pos);
        let end = find_closing_parens(&s, pos + key.len()).unwrap_or(pos);

        if &s[start..start+1] == "{" && &s[end..end+1] == "}" {
            s = String::from(&s[..start]) + &s[end+1..];
        } else {
            s = String::from(&s[..pos]) + &s[pos+key.len()..]
        }
    }
    s
}

fn sum_of_numbers(obj: &json::JsonValue) -> i32 {
    use std::convert::TryInto;
    match obj {
        json::JsonValue::Number(n) => n.clone().try_into().unwrap_or_else(|_| panic!()),
        json::JsonValue::Object(obj) => obj.iter().map(|(_, v)| sum_of_numbers(v)).sum(),
        json::JsonValue::Array(v) => v.iter().map(sum_of_numbers).sum(),
        _ => 0
    }
}

fn sum_excluding_values(obj: &json::JsonValue, key: &str) -> i32 {
    use std::convert::TryInto;
    match obj {
        json::JsonValue::Number(n) => n.clone().try_into().unwrap_or_else(|_| panic!()),
        json::JsonValue::Array(v) => v.iter().map(|obj| sum_excluding_values(obj, key)).sum(),
        json::JsonValue::Object(obj) =>
            if obj.iter().find(|(_, v)| *v == key).is_some() {
                0
            } else {
                obj.iter().map(|(_, v)| sum_excluding_values(v, key)).sum()
            }
        _ => 0
    }
}

struct Numbers<'a> {
    s: &'a str,
}

impl<'a> Iterator for Numbers<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        let pos = self.s.find(|c: char| c == '-' || c.is_ascii_digit());
        if let Some(pos) = pos {
            self.s = &self.s[pos..];
            let mut c = self.s.chars();
            match c.next() {
                None => None,
                Some('-') => {
                    self.s = &self.s[1..];
                    match c.next() {
                        None | Some('-') => {
                            self.next()
                        }
                        Some(_) => {
                            let (n, len) = parse_number_at_start(self.s);
                            self.s = &self.s[len..];
                            Some(-n)
                        }
                    }
                }
                Some(_) => {
                    let (n, len) = parse_number_at_start(self.s);
                    self.s = &self.s[len..];
                    Some(n)
                }
            }
        } else {
            None
        }
    }
}

fn parse_number_at_start(s: &str) -> (i32, usize) {
    assert!(s.len() > 0);
    assert!(s.chars().next().unwrap().is_ascii_digit());
    let endpos = s.find(|c: char| !c.is_ascii_digit()).unwrap_or(s.len());
    assert!(endpos > 0);
    let n = s[..endpos].parse().unwrap();
    (n, endpos)
}

fn find_opening_parens(s: &str, pos: usize) -> Option<usize> {
    let mut depth = 0;
    for (p, c) in s[..pos].char_indices().rev() {
        depth += match c {
            '{' | '[' => -1,
            '}' | ']' => 1,
            _ => 0,
        };
        if depth == -1 {
            return Some(p);
        }
    }
    None
}

fn find_closing_parens(s: &str, pos: usize) -> Option<usize> {
    let mut depth = 0;
    for (p, c) in s[pos..].char_indices() {
        depth += match c {
            '{' | '[' => 1,
            '}' | ']' => -1,
            _ => 0,
        };
        if depth == -1 {
            return Some(pos + p);
        }
    }
    None
}

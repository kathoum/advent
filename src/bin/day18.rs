fn main() {
    let input = include_str!("input18.txt");
    
    println!("Part One");
    // println!("{} = 71", evaluate_formula("1 + 2 * 3 + 4 * 5 + 6"));
    // println!("{} = 51", evaluate_formula("1 + (2 * 3) + (4 * (5 + 6))"));
    // println!("{} = 26", evaluate_formula("2 * 3 + (4 * 5)"));
    // println!("{} = 437", evaluate_formula("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
    // println!("{} = 12240", evaluate_formula("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
    // println!("{} = 13632", evaluate_formula("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    let total: i64 = input.lines().map(evaluate_formula).sum();
    println!("Sum of results is {}", total);

    println!("Part Two");
    // println!("{} = 231", evaluate_advanced_formula("1 + 2 * 3 + 4 * 5 + 6"));
    // println!("{} = 51", evaluate_advanced_formula("1 + (2 * 3) + (4 * (5 + 6))"));
    // println!("{} = 46", evaluate_advanced_formula("2 * 3 + (4 * 5)"));
    // println!("{} = 1445", evaluate_advanced_formula("5 + (8 * 3 + 9 + 3 * 4 * 3)"));
    // println!("{} = 669060", evaluate_advanced_formula("5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))"));
    // println!("{} = 23340", evaluate_advanced_formula("((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2"));
    let total: i64 = input.lines().map(evaluate_advanced_formula).sum();
    println!("Sum of results is {}", total);
}

#[derive(Copy, Clone, Debug)]
enum State {
    Start,            // ""
    StartN(i64),      // "12"
    Number(i64),      // "12 " or "(...)"
    Plus(i64),        // "... +"
    PlusN(i64, i64),  // "... + 12"
    Times(i64),       // "... *"
    TimesN(i64, i64), // "... * 12"
}

fn evaluate_formula(s: &str) -> i64 {
    let mut stack = vec![State::Start];
    process_char(&mut stack, '(');
    for c in s.chars() {
        process_char(&mut stack, c);
        //println!("{} -> {:?}", c, stack.last());
    }
    process_char(&mut stack, ')');

    assert_eq!(stack.len(), 1, "Unclosed parenthesis");
    if let State::Number(n) = stack[0] {
        n
    } else {
        panic!("Incomplete expression")
    }
}

fn process_char(stack: &mut Vec<State>, c: char) {
    enum CC { Digit(i64), Plus, Times, Open, Close, Space }
    let next = match c {
        '0'..='9' => CC::Digit(c.to_digit(10).unwrap() as i64),
        '+' => CC::Plus,
        '*' => CC::Times,
        '(' => CC::Open,
        ')' => CC::Close,
        _ if c.is_ascii_whitespace() => CC::Space,
        _ => panic!("Invalid character {}", c)
    };

    enum Action {
        Nop,
        Replace(State),
        Push,
        Pop(i64),
    }

    let action = match (*stack.last().unwrap(), next) {
        // ""
        (State::Start, CC::Digit(d)) => Action::Replace(State::StartN(d)),
        (State::Start, CC::Open) => Action::Push,
        (State::Start, CC::Space) => Action::Nop,
        // "12" or "12 " or "(...)"
        (State::StartN(n), CC::Digit(d)) => Action::Replace(State::StartN(n * 10 + d)),
        (State::StartN(n), CC::Plus) => Action::Replace(State::Plus(n)),
        (State::StartN(n), CC::Times) => Action::Replace(State::Times(n)),
        (State::StartN(n), CC::Close) => Action::Pop(n),
        (State::StartN(n), CC::Space) => Action::Replace(State::Number(n)),
        // "12 " or "(...)"
        (State::Number(n), CC::Plus) => Action::Replace(State::Plus(n)),
        (State::Number(n), CC::Times) => Action::Replace(State::Times(n)),
        (State::Number(n), CC::Close) => Action::Pop(n),
        (State::Number(_), CC::Space) => Action::Nop,
        // "... +"
        (State::Plus(n), CC::Digit(d)) => Action::Replace(State::PlusN(n, d)),
        (State::Plus(_), CC::Open) => Action::Push,
        (State::Plus(_), CC::Space) => Action::Nop,
        // "... + 12"
        (State::PlusN(n, m), CC::Digit(d)) => Action::Replace(State::PlusN(n, m * 10 + d)),
        (State::PlusN(n, m), CC::Plus) => Action::Replace(State::Plus(n + m)),
        (State::PlusN(n, m), CC::Times) => Action::Replace(State::Times(n + m)),
        (State::PlusN(n, m), CC::Close) => Action::Pop(n + m),
        (State::PlusN(n, m), CC::Space) => Action::Replace(State::Number(n + m)),
        // "... *"
        (State::Times(n), CC::Digit(d)) => Action::Replace(State::TimesN(n, d)),
        (State::Times(_), CC::Open) => Action::Push,
        (State::Times(_), CC::Space) => Action::Nop,
        // "... * 12"
        (State::TimesN(n, m), CC::Digit(d)) => Action::Replace(State::TimesN(n, m * 10 + d)),
        (State::TimesN(n, m), CC::Plus) => Action::Replace(State::Plus(n * m)),
        (State::TimesN(n, m), CC::Times) => Action::Replace(State::Times(n * m)),
        (State::TimesN(n, m), CC::Close) => Action::Pop(n * m),
        (State::TimesN(n, m), CC::Space) => Action::Replace(State::Number(n * m)),

        _ => panic!("Unexpected character {}", c)
    };

    match action {
        Action::Nop => (),
        Action::Replace(new_state) =>
            *stack.last_mut().unwrap() = new_state,
        Action::Push =>
            stack.push(State::Start),
        Action::Pop(n) => {
            stack.pop().expect("Unbalanced parenthesis");
            let value = match stack.pop() {
                Some(State::Start) => n,
                Some(State::Plus(m)) => m + n,
                Some(State::Times(m)) => m * n,
                _ => panic!()
            };
            stack.push(State::Number(value));
        }
    }
}

fn evaluate_advanced_formula(s: &str) -> i64 {
    if let Some(open) = s.rfind('(') {
        match s[open..].find(')') {
            None => panic!("Unclosed parenthesis"),
            Some(close) => {
                let close = open + close;
                let n = evaluate_advanced_formula(&s[open+1..close]);
                let t = s[..open].to_string() + &n.to_string() + &s[close+1..];
                evaluate_advanced_formula(&t)
            }
        }
    } else if s.contains('*') {
        s.split('*').map(|factor| evaluate_advanced_formula(factor)).product()
    } else if s.contains('+') {
        s.split('+').map(|addend| evaluate_advanced_formula(addend)).sum()
    } else {
        s.trim().parse().unwrap()
    }
}

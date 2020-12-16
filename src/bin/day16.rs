use std::ops::RangeInclusive;

struct Rule {
    field_name: &'static str,
    range1: RangeInclusive<i32>,
    range2: RangeInclusive<i32>,
}

impl Rule {
    fn check(&self, value: i32) -> bool {
        self.range1.contains(&value) || self.range2.contains(&value)
    }
}

fn main() {
    let input = include_str!("input16.txt");

    let rules: Vec<Rule> = input.lines()
        .take_while(|&s| s != "")
        .map(|line| {
            let mut part = line.split(": ");
            let field_name = part.next().unwrap();
            let rest = part.next().unwrap();
            let (n1, n2, n3, n4): (i32, i32, i32, i32);
            text_io::scan!(rest.bytes() => "{}-{} or {}-{}", n1, n2, n3, n4);
            Rule { field_name, range1: n1..=n2, range2: n3..=n4 }
        })
        .collect();

    let my_ticket: Vec<i32> = input.lines()
        .skip(rules.len() + 2).next().unwrap()
        .split(',').map(|n| n.parse().unwrap())
        .collect();

    let nearby_tickets: Vec<Vec<i32>> = input.lines()
        .skip(rules.len() + 5)
        .map(|line| {
            line.split(',').map(|n| n.parse().unwrap()).collect()
        })
        .collect();

    println!("Part One");
    let invalid_values = nearby_tickets.iter().flatten()
        .filter(|n| rules.iter().all(|r| !r.check(**n)));
    println!("Ticket scanning error rate is {}", invalid_values.sum::<i32>());

    println!("Part Two");
    let valid_tickets: Vec<Vec<i32>> = nearby_tickets.into_iter()
        .filter(|ticket| ticket.iter().all(|n| rules.iter().any(|r| r.check(*n))))
        .collect();
    
    // Compatibility matrix:
    // matrix[rule][pos] = 'rule' can be in position 'pos'
    let rows = rules.len();
    let cols = my_ticket.len();
    let mut matrix: Vec<Vec<bool>> = rules.iter()
        .map(|rule| {
            (0..cols).map(|pos| {
                valid_tickets.iter().all(|ticket| rule.check(ticket[pos]))
            }).collect()
        }).collect();

    let mut num_fixed_rules = 0;
    while num_fixed_rules < rows {
        // Find a row with only one 'true'
        num_fixed_rules = 0;
        for row in 0..rows {
            match matrix[row].iter().filter(|x| **x).count() {
                0 => panic!("No solution possible"),
                1 => {
                    num_fixed_rules += 1;
                    // Set all other entries in the same column to false
                    for col in 0..cols {
                        if matrix[row][col] {
                            for r in 0..rows {
                                if r != row {
                                    matrix[r][col] = false;
                                }
                            }
                        }
                    }
                },
                _ => ()
            }
        }
        // Find a column with only one 'true'
        let mut num_fixed_fields = 0;
        for col in 0..cols {
            match matrix.iter().filter(|r| r[col]).count() {
                0 => panic!("No solution possible"),
                1 => {
                    num_fixed_fields += 1;
                    // Set all other entries in the same row to false
                    for row in 0..rows {
                        if matrix[row][col] {
                            for c in 0..cols {
                                if c != col {
                                    matrix[row][c] = false;
                                }
                            }
                        }
                    }
                },
                _ => ()
            }
        }
        println!("Next iteration: {} rules and {} fields assigned", num_fixed_rules, num_fixed_fields);
    }

    let fields: Vec<usize> = matrix.iter().map(|row| {
        row.iter().enumerate().find(|(_, value)| **value).unwrap().0
    }).collect();

    let mut product = 1i64;
    for (rule, field) in rules.iter().zip(fields.iter()) {
        if rule.field_name.starts_with("departure") {
            println!("{}: {}", rule.field_name, my_ticket[*field]);
            product *= my_ticket[*field] as i64;
        }
    }
    println!("The product of my fields is {}", product);
}
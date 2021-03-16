fn main() {
    let input = include_str!("input05.txt");

    println!("Part one: there are {} nice strings", input.lines().filter(is_nice_1).count());
    println!("Part two: there are {} nice strings", input.lines().filter(is_nice_2).count());
}

fn is_nice_1(str: &&str) -> bool {
    str.matches(|c| "aeiou".contains(c)).count() >= 3 &&
    str.chars().zip(str.chars().skip(1)).find(|(c1, c2)| c1 == c2).is_some() &&
    !str.contains("ab") &&
    !str.contains("cd") &&
    !str.contains("pq") &&
    !str.contains("xy")
}

fn is_nice_2(str: &&str) -> bool {
    let mut f1 = false;
    let mut f2 = false;
    let mut chars = str.chars();
    loop {
        let str = chars.as_str();
        chars.next();
        if str.is_empty() {
            break
        }

        let mut ci = str.char_indices();
        let c1 = ci.next();
        ci.next();
        let rest = ci.as_str();
        let c3 = ci.next();
        if let Some((i3, c3)) = c3 {
            if rest.contains(&str[..i3]) {
                f1 = true;
            }
            if matches!(c1, Some((_, c)) if c == c3) {
                f2 = true;
            }
        }
    }
    f1 && f2
}

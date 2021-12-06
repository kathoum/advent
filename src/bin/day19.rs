use std::collections::HashSet;

fn main() {
    let input = include_str!("input19.txt");

    let mut formulas = Vec::new();
    let mut molecule = None;
    for line in input.lines() {
        if line.contains("=>") {
            let mut iter = line.split("=>");
            let a = iter.next().unwrap().trim();
            let b = iter.next().unwrap().trim();
            formulas.push((a, b));
        } else {
            let l = line.trim();
            if !l.is_empty() {
                molecule = Some(l);
            }
        }
    }
    let molecule = molecule.unwrap();

    let products = get_products(formulas.as_slice(), molecule);
    println!("Part one: {} possible outputs", products.len());

    // Reverse the formulas and sort by how much they shorten the string
    let mut reverse_formulas: Vec<_> = formulas.iter().map(|&(a,b)| (b,a)).collect();
    reverse_formulas.sort_by_key(|(a,b)| b.len() as isize - a.len() as isize);
    let l = path_length(reverse_formulas.as_slice(), molecule.into(), "e".into());
    println!("Part two: {} replacements necessary", l.unwrap());
}

fn get_products(formulas: &[(&str, &str)], molecule: &str) -> HashSet<String> {
    let mut products = HashSet::new();
    for formula in formulas.iter() {
        for (pos, s) in molecule.match_indices(formula.0) {
            products.insert(format!("{}{}{}", &molecule[..pos], formula.1, &molecule[pos + s.len()..]));
        }
    }
    products
}

fn path_length(formulas: &[(&str, &str)], from: &str, to: &str) -> Option<usize> {
    if from == to {
       return Some(0); 
    }
    for formula in formulas {
        for (pos, s) in from.match_indices(formula.0) {
            let next = format!("{}{}{}", &from[..pos], formula.1, &from[pos + s.len()..]);
            if let Some(n) = path_length(formulas, next.as_str(), to) {
                return Some(n + 1);
            }
        }
    }
    None
}

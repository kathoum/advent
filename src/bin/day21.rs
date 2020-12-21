use std::collections::{HashMap, HashSet};

type Ingredient = String;
type Allergen = String;

fn main() {
    let input = include_str!("input21.txt");

    let recipes: Vec<(Vec<Ingredient>, Vec<Allergen>)> = input.lines().map(|line| {
        let paren = line.find('(').unwrap();
        let ingr: Vec<Ingredient> = line[..paren].split_whitespace().map(String::from).collect();
        let allerg: Vec<Allergen> = line[paren..]
            .strip_prefix("(contains").unwrap()
            .split(&[' ', ',', ')'][..])
            .filter(|s| !s.is_empty())
            .map(String::from).collect();
        (ingr, allerg)
    }).collect();

    println!("Part One");
    let mut possibilities: HashMap<Allergen, HashSet<Ingredient>> = HashMap::new();
    for (ingredients, allergens) in recipes.iter() {
        let ingr: HashSet<Ingredient> = ingredients.iter().map(String::from).collect();
        for allerg in allergens.iter() {
            let new_ingr = match possibilities.get(allerg) {
                None => ingr.clone(),
                Some(existing) => &ingr & existing,
            };
            possibilities.insert(allerg.to_string(), new_ingr);
        }
    }

    let mut pairs: Vec<(Allergen, Ingredient)> = Vec::new();
    while !possibilities.is_empty() {
        let determined = possibilities.iter().find(|(allergen, ingredients)|
            if ingredients.len() == 0 {
                panic!("No possibility for {}", allergen)
            } else {
                ingredients.len() == 1
            }
        ).expect(&format!("Unsolvable ambiguity: {:?}", possibilities));
        let allergen = determined.0.clone();
        let ingredient = determined.1.iter().next().unwrap().clone();
        pairs.push((allergen.clone(), ingredient.clone()));
        println!("{} is in {}", allergen, ingredient);
        possibilities.remove(&allergen);
        for (_, value) in possibilities.iter_mut() {
            value.remove(&ingredient);
        }
    }

    let count: usize = recipes.iter().map(|(ingr, _)|
        ingr.iter()
            .filter(|&ingr| pairs.iter().find(|p| p.1 == *ingr).is_none())
            .count()
    ).sum();
    println!("{} ingredients without allergens", count);

    println!("Part Two");
    pairs.sort();
    let canonical_list = pairs.iter()
        .map(|p| p.1.as_str())
        .collect::<Vec<_>>()
        .join(",");
    println!("The canonical dangerous ingredient list is: {}", canonical_list);
}

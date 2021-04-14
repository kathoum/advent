fn main() {
    let input = include_str!("input15.txt");
    let ingredients: Vec<Ingredient> = input.lines().map(parse_ingredient).collect();

    let mut max_score = 0;
    let mut max_score_500 = 0;

    let mut range = NdRange::new(100, ingredients.len());
    while let Some(quantities) = range.next() {
        let (score, calories) = score(&dot(&ingredients, quantities));
        max_score = max_score.max(score);
        if calories == 500 {
            max_score_500 = max_score_500.max(score);
        }
    }

    println!("Total score is {}", max_score);
    println!("For 500 calories, total score is {}", max_score_500);
}

type Str = &'static str;

#[derive(Clone)]
struct Ingredient {
    name: Str,
    attributes: Vec<(Str, i32)>,
}

fn parse_ingredient(input: Str) -> Ingredient {
    let mut iter = input.splitn(2, ": ");
    let name = iter.next().unwrap();
    let rest = iter.next().unwrap();
    let attributes = rest.split(", ").map(|s| {
        let mut it = s.splitn(2, ' ');
        let word = it.next().unwrap();
        let number = it.next().unwrap();
        (word, number.parse().unwrap())
    }).collect();
    Ingredient { name, attributes }
}

impl std::ops::MulAssign<i32> for Ingredient {
    fn mul_assign(&mut self, n: i32) {
        for attr in self.attributes.iter_mut() {
            attr.1 *= n;
        }
    }
}

impl std::ops::Mul<i32> for Ingredient {
    type Output = Ingredient;
    fn mul(mut self, n: i32) -> Ingredient { self *= n; self }
}

impl std::ops::Mul<i32> for &Ingredient {
    type Output = Ingredient;
    fn mul(self, n: i32) -> Ingredient { self.clone() * n }
}

impl std::ops::AddAssign<&Ingredient> for Ingredient {
    fn add_assign(&mut self, other: &Ingredient) {
        assert_eq!(self.attributes.len(), other.attributes.len());
        for (a, b) in self.attributes.iter_mut().zip(other.attributes.iter()) {
            assert_eq!(a.0, b.0);
            a.1 += b.1;
        }
    }
}

impl std::ops::Add<&Ingredient> for Ingredient {
    type Output = Ingredient;
    fn add(mut self, other: &Ingredient) -> Ingredient { self += other; self }
}

impl std::ops::Add<&Ingredient> for &Ingredient {
    type Output = Ingredient;
    fn add(self, other: &Ingredient) -> Ingredient { self.clone() + other }
}

fn dot(ingr: &[Ingredient], q: &[i32]) -> Ingredient {
    assert_eq!(ingr.len(), q.len());
    ingr.iter().zip(q.iter()).fold(None, |acc, (ingredient, &quantity)| {
        match acc {
            None => Some(ingredient * quantity),
            Some(a) => Some(a + &(ingredient * quantity)),
        }
    }).unwrap()
}

fn score(ingr: &Ingredient) -> (i32, i32) {
    let mut value = 1;
    let mut calories = None;
    for (name, n) in ingr.attributes.iter() {
        if name == &"calories" {
            calories = Some(*n);
        } else {
            value *= 0.max(*n);
        }
    }
    (value, calories.unwrap())
}

/// Generates all n-tuples of non-negative integers with a given sum
struct NdRange {
    values: Vec<i32>,
}

impl NdRange {
    fn new(total: i32, dims: usize) -> NdRange {
        assert!(dims > 1);
        let mut values = vec![0; dims];
        values[0] = -1;
        values[dims-1] = total + 1;
        NdRange { values }
    }

    fn next(&mut self) -> Option<&[i32]> {
        let (last, values) = self.values.split_last_mut().unwrap();
        for n in values.iter_mut() {
            if *last > 0 {
                *n += 1;
                *last -= 1;
                return Some(self.values.as_slice());
            } else {
                *last += *n;
                *n = 0;
            }
        }
        None
    }
}

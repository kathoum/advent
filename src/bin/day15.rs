fn main() {
    let input = include_str!("input15.txt");
    let ingredients: Vec<Ingredient> = input.lines().map(parse_ingredient).collect();
    assert_eq!(ingredients.len(), 4);
    let mut max_score = 0;
    let mut max_score_500 = 0;
    for q1 in 0..=100 {
        for q2 in 0..=100-q1 {
            for q3 in 0..=100-q1-q2 {
                let q = [q1, q2, q3, 100-q1-q2-q3];
                let s = score(&ingredients, &q);
                max_score = max_score.max(s.0);
                if s.1 == 500 {
                    max_score_500 = max_score_500.max(s.0);
                }
            }
        }
    }
    println!("Total score is {}", max_score);
    println!("For 500 calories, total score is {}", max_score_500);
}

type Ingredient = [i32; 5];

fn parse_ingredient(input: &str) -> Ingredient {
    let _name: String;
    let (i1, i2, i3, i4, i5);
    text_io::scan!(input.bytes() => "{}: capacity {}, durability {}, flavor {}, texture {}, calories {}",
        _name, i1, i2, i3, i4, i5);
    [i1, i2, i3, i4, i5]
}

fn score(ingr: &[Ingredient], q: &[i32]) -> (i32, i32) {
    let mut x = [0; 5];
    for (j, r) in ingr.iter().zip(q.iter()) {
        for k in 0..5 {
            x[k] += j[k] * r;
        }
    }
    (x[0..4].iter().map(|n| n.max(&0)).product(), x[4])
}

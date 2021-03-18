
use std::collections::HashSet;

fn main() {
    let input = include_str!("input09.txt");
    let legs: Vec<Leg> = input.lines().map(|s| s.parse()).collect::<Result<_,_>>().unwrap();
    let mut places = HashSet::<String>::new();
    for l in legs.iter() {
        places.insert(l.0.clone());
        places.insert(l.1.clone());
    }

    let distances = get_all_routes(&legs, &places, &mut Vec::new(), 0, Vec::new());
    println!("The shortest route has length {}", distances.iter().min().unwrap());
    println!("The longest route has length {}", distances.iter().max().unwrap());
}

struct Leg(String, String, i32);

fn get_all_routes(legs: &Vec<Leg>, places: &HashSet<String>, visited: &mut Vec<String>, dist: i32, mut distances: Vec<i32>) -> Vec<i32> {
    if places.len() > visited.len() {
        match visited.last() {
            None => {
                for start in places {
                    visited.push(start.clone());
                    distances = get_all_routes(legs, places, visited, dist, distances);
                    visited.pop();
                }
                distances
            }
            Some(s) => {
                let start = s.clone();
                for leg in legs.iter() {
                    if leg.0 == start && !visited.contains(&leg.1) {
                        visited.push(leg.1.clone());
                        distances = get_all_routes(legs, places, visited, dist + leg.2, distances);
                        visited.pop();
                    }
                    if leg.1 == start && !visited.contains(&leg.0) {
                        visited.push(leg.0.clone());
                        distances = get_all_routes(legs, places, visited, dist + leg.2, distances);
                        visited.pop();
                    }
                }
                distances
            }
        }
    } else {
        distances.push(dist);
        distances
    }
}

impl std::str::FromStr for Leg {
    type Err = ();
    fn from_str(str: &str) -> Result<Leg, ()> {
        let p1 = str.find(" to ").ok_or(())?;
        let p2 = str.find(" = ").ok_or(())?;
        Ok(Leg(
            str[..p1].into(),
            str[p1+4..p2].into(),
            str[p2+3..].parse().or_else(|_| Err(()))?
        ))
    }
}

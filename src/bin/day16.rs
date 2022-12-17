use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;

#[derive(Clone)]
struct Valve {
    pub name: String,
    pub flow: i32,
    pub starting_distance: i32,
}

struct Input {
    pub valves: Vec<Valve>,
    pub distances: Vec<Vec<i32>>,
}

fn main() {
    let reader = BufReader::new(File::open("input/day16.txt").unwrap());
    let input = parse(reader, "AA");

    let score = find_best_score(&input, 30, 1);
    println!("The maximum releasable pressure is {score}");

    let score = find_best_score(&input, 26, 2);
    println!("The maximum releasable pressure with the help of the elephant is {score}");
}

fn find_best_score(input: &Input, max_time: i32, num_agents: u32) -> i32 {
    continue_search(input, max_time, &mut vec![false; input.valves.len()], None, num_agents, 0, 0)
}

fn continue_search(input: &Input, max_time: i32, visited: &mut[bool], last: Option<usize>, restarts: u32, remaining_time: i32, score: i32) -> i32 {
    let mut best_score = score;
    for (i,valve) in input.valves.iter().enumerate() {
        if !visited[i] {
            if let Some(last) = last {
                let t = remaining_time - input.distances[last][i] - 1;
                if t > 0 {
                    visited[i] = true;
                    let new_best = continue_search(input, max_time, visited, Some(i), restarts, t, score + t * valve.flow);
                    visited[i] = false;
                    best_score = best_score.max(new_best);
                }
            }
            if restarts > 0 {
                let t = max_time - valve.starting_distance - 1;
                if t > 0 {
                    visited[i] = true;
                    let new_best = continue_search(input, max_time, visited, Some(i), restarts - 1, t, score + t * valve.flow);
                    visited[i] = false;
                    best_score = best_score.max(new_best);
                }
            }
        }
    }
    best_score
}

fn parse(reader: impl BufRead, starting_valve: &str) -> Input {
    let mut all_valves = Vec::new();
    let mut useful_valves = Vec::new();
    let mut idx: HashMap<String,(usize,usize)> = HashMap::new();
    let mut edge_names: Vec<Vec<String>> = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let words = line.split([' ', '=', ';', ',']).collect::<Vec<&str>>();
        let v = Valve {
            name: words[1].to_owned(),
            flow: words[5].parse().unwrap(),
            starting_distance: -1,
        };

        idx.insert(v.name.to_owned(), (all_valves.len(), useful_valves.len()));
        if v.flow > 0 {
            useful_valves.push(v.clone());
        }
        all_valves.push(v);
        edge_names.push(words[11..].iter().step_by(2).map(|s| (*s).to_owned()).collect());
    }

    let edges = edge_names.into_iter().map(|v| v.into_iter().map(|n| idx[&n].0).collect()).collect();

    for (i,d) in distances_from(&edges, idx[starting_valve].0).into_iter().enumerate() {
        if all_valves[i].flow > 0 {
            useful_valves[idx[&all_valves[i].name].1].starting_distance = d;
        }
    }
    let distances: Vec<Vec<i32>> = useful_valves.iter().map(|v| {
        let all_dist = distances_from(&edges, idx[&v.name].0);
        all_dist.into_iter().enumerate().filter_map(|(i,d)| if all_valves[i].flow > 0 { Some(d) } else { None }).collect()
    }).collect();

    assert!(useful_valves.iter().all(|v| v.flow > 0 && v.starting_distance >= 0));
    assert!(distances.len() == useful_valves.len());
    assert!(distances.iter().all(|d| d.len() == useful_valves.len()));

    Input { valves: useful_valves, distances }
}

fn distances_from(edges: &Vec<Vec<usize>>, a: usize) -> Vec<i32> {
    let mut d = vec![-1; edges.len()];
    let mut l = vec![a];
    d[a] = 0;
    while !l.is_empty() {
        let mut l2 = Vec::new();
        for x in l {
            for &e in &edges[x] {
                if d[e] < 0 {
                    l2.push(e);
                    d[e] = d[x] + 1;
                }
            }
        }
        l = l2;
    }
    assert!(d.iter().all(|&n| n >= 0));
    d
}

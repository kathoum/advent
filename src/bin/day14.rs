fn main() {
    let input = include_str!("input14.txt");
    let reindeers: Vec<Reindeer> = input.lines().map(parse).collect();

    let time = 2503;
    let distances: Vec<u32> = reindeers.iter().map(|r| distance(r, time)).collect();
    let best = reindeers.iter().zip(distances.iter()).max_by_key(|p| p.1).unwrap();
    println!("The fastest reindeer after {} seconds is {} with {} km", time, best.0.name, best.1);

    let mut distances = vec![0u32; distances.len()];
    let mut scores = vec![0u32; distances.len()];
    for t in 1..=time {
        for (d, r) in distances.iter_mut().zip(reindeers.iter()) {
            *d = distance(r, t);
        }
        let max = distances.iter().max().unwrap();
        for (s, d) in scores.iter_mut().zip(distances.iter()) {
            if d == max {
                *s += 1;
            }
        }
    }
    let best = reindeers.iter().zip(scores.iter()).max_by_key(|p| p.1).unwrap();
    println!("The reindeer with most points is {} with {}", best.0.name, best.1);
}

struct Reindeer {
    name: String,
    speed: u32,
    sprint: u32,
    rest: u32,
}

fn parse(line: &str) -> Reindeer {
    let (name, speed, sprint, rest);
    text_io::scan!(line.bytes() => "{} can fly {} km/s for {} seconds, but then must rest for {} seconds.", name, speed, sprint, rest);
    Reindeer { name, speed, sprint, rest }
}

fn distance(r: &Reindeer, t: u32) -> u32 {
    let cycles = t / (r.sprint + r.rest);
    let remainder = (t % (r.sprint + r.rest)).min(r.sprint);
    (cycles * r.sprint + remainder) * r.speed
}

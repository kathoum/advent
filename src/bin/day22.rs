use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let bricks: Vec<Brick> = BufReader::new(File::open("input/day22.txt").unwrap())
        .lines()
        .map(|line| line.unwrap().parse().unwrap())
        .collect();
    let mut game = Game::new(bricks);

    game.fall();
    let answer = game.bricks.len() - game.num_essential_bricks();
    println!("Day 22 Part One: {answer}");

    let supporting: Vec<HashSet<usize>> = game
        .bricks
        .iter()
        .map(|brick| game.supporting_bricks(brick))
        .collect();
    let answer: usize = (0..game.bricks.len())
        .map(|brick| game.num_supported_bricks(&supporting, brick))
        .sum();
    println!("Day 22 Part Two: {answer}");
}

#[derive(Debug, Clone, Copy)]
struct Brick {
    x: (i32, i32),
    y: (i32, i32),
    z: (i32, i32),
}

impl Brick {
    fn coords(&self) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        (self.x.0..=self.x.1).flat_map(move |x| {
            (self.y.0..=self.y.1)
                .flat_map(move |y| (self.z.0..=self.z.1).map(move |z| (x as _, y as _, z as _)))
        })
    }

    fn support(&self) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        (self.x.0..=self.x.1).flat_map(move |x| {
            (self.y.0..=self.y.1).map(move |y| (x as _, y as _, self.z.0 as usize - 1))
        })
    }
}

struct Game {
    bricks: Vec<Brick>,
    space: Vec<Vec<Vec<usize>>>,
}

impl Game {
    fn new(mut bricks: Vec<Brick>) -> Self {
        bricks.sort_by_key(|b| b.z.0);
        let (x, y, z) = bricks.iter().fold((0, 0, 0), |a, b| {
            (a.0.max(b.x.1), a.1.max(b.y.1), a.2.max(b.z.1))
        });
        let mut space =
            vec![vec![vec![usize::MAX; 1 + z as usize]; 1 + y as usize]; 1 + x as usize];
        for (i, brick) in bricks.iter().enumerate() {
            for (x, y, z) in brick.coords() {
                space[x][y][z] = i;
            }
        }
        Game { bricks, space }
    }

    fn fall(&mut self) {
        for brick in &mut self.bricks {
            while brick.z.0 > 1
                && brick
                    .support()
                    .all(|(x, y, z)| self.space[x][y][z] == usize::MAX)
            {
                let mut i = 0;
                for (x, y, z) in brick.coords() {
                    i = std::mem::replace(&mut self.space[x][y][z], usize::MAX);
                }
                brick.z.0 -= 1;
                brick.z.1 -= 1;
                for (x, y, z) in brick.coords() {
                    self.space[x][y][z] = i;
                }
            }
        }
    }

    fn supporting_bricks(&self, brick: &Brick) -> HashSet<usize> {
        brick
            .support()
            .map(|(x, y, z)| self.space[x][y][z])
            .filter(|&s| s != usize::MAX)
            .collect()
    }

    fn num_essential_bricks(&self) -> usize {
        let result: HashSet<usize> = self
            .bricks
            .iter()
            .filter_map(|brick| {
                let s = self.supporting_bricks(brick);
                (s.len() == 1).then(|| s.into_iter().next().unwrap())
            })
            .collect();
        result.len()
    }

    fn num_supported_bricks(&self, supporting: &[HashSet<usize>], brick: usize) -> usize {
        let mut deleted = HashSet::from([brick]);
        let mut exit = false;
        while !exit {
            exit = true;
            for (i, set) in supporting.iter().enumerate() {
                if !set.is_empty() && !deleted.contains(&i) && set.is_subset(&deleted) {
                    deleted.insert(i);
                    exit = false;
                }
            }
        }
        deleted.len() - 1
    }
}

impl FromStr for Brick {
    type Err = std::num::ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut n = s.split([',', '~']);
        let x0: i32 = n.next().unwrap().parse()?;
        let y0: i32 = n.next().unwrap().parse()?;
        let z0: i32 = n.next().unwrap().parse()?;
        let x1: i32 = n.next().unwrap().parse()?;
        let y1: i32 = n.next().unwrap().parse()?;
        let z1: i32 = n.next().unwrap().parse()?;
        Ok(Brick {
            x: (x0.min(x1), x0.max(x1)),
            y: (y0.min(y1), y0.max(y1)),
            z: (z0.min(z1), z0.max(z1)),
        })
    }
}

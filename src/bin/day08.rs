use std::fs::File;
use std::io::{BufRead,BufReader};

fn main() {
    let reader = BufReader::new(File::open("input/day08.txt").unwrap());

    let grid = Grid::from(reader.lines()
        .map(Result::unwrap)
        .map(String::into_bytes)
        .collect());

    let (num_visible, max_scenic_score) = grid.range()
        .map(|i| grid.score(i))
        .fold((0,0), |(n,max), score| (
            n + score.is_visible as u32,
            max.max(score.scenic_score)
        ));

    println!("There are {num_visible} visible trees");
    println!("The maximum scenic score is {max_scenic_score}");
}

struct Grid(Vec<Vec<u8>>);
struct Score {
    is_visible: bool,
    scenic_score: u32,
}

impl Grid {
    pub fn from(data: Vec<Vec<u8>>) -> Self {
        assert!(data.iter().all(|v| v.len() == data[0].len()));
        Self(data)
    }

    pub fn rows(&self) -> usize { self.0.len() }
    pub fn cols(&self) -> usize { self.0[0].len() }
    pub fn range(&self) -> impl Iterator<Item = (usize,usize)> + '_ {
        (0..self.rows()).flat_map(|i| (0..self.cols()).map(move |j| (i,j)))
    }

    pub fn score(&self, idx: (usize,usize)) -> Score {
        let (i,j) = idx;
        let h = self[idx];
        let s1 = score_along_line(h, (0..i).rev().map(|i| self[(i,j)]));
        let s2 = score_along_line(h, (i+1..self.rows()).map(|i| self[(i,j)]));
        let s3 = score_along_line(h, (0..j).rev().map(|j| self[(i,j)]));
        let s4 = score_along_line(h, (j+1..self.cols()).map(|j| self[(i,j)]));
        Score {
            is_visible: s1.is_visible || s2.is_visible || s3.is_visible || s4.is_visible,
            scenic_score: s1.scenic_score * s2.scenic_score * s3.scenic_score * s4.scenic_score,
        }
    }
}

impl std::ops::Index<(usize,usize)> for Grid {
    type Output = u8;
    fn index(&self, idx: (usize,usize)) -> &Self::Output {
        &self.0[idx.0][idx.1]
    }
}

fn score_along_line(limit: u8, iter: impl IntoIterator<Item = u8>) -> Score {
    let mut count = 0;
    for n in iter {
        count += 1;
        if n >= limit {
            return Score { is_visible: false, scenic_score: count };
        }
    }
    Score { is_visible: true, scenic_score: count }
}

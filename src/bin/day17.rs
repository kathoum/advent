use std::fs::File;
use std::io::{BufRead,BufReader};

struct Tetris {
    columns: [Vec<bool>; 7],
    piece: Option<&'static [(i64,i64)]>,
    pos: (i64,i64),
}

const PIECES: &[&[(i64,i64)]] = &[
    /* - */ &[(0,0), (1,0), (2,0), (3,0)],
    /* + */ &[(1,0), (0,1), (1,1), (2,1), (1,2)],
    /* L */ &[(0,0), (1,0), (2,0), (2,1), (2,2)],
    /* | */ &[(0,0), (0,1), (0,2), (0,3)],
    /* o */ &[(0,0), (0,1), (1,0), (1,1)],
];

fn main() {
    let reader = BufReader::new(File::open("input/day17.txt").unwrap());
    let jets = reader.lines().next().unwrap().unwrap();

    let piece_count = 2022;
    let piece_count_long: usize = 1000000000000;

    let mut tetris = Tetris::new();
    let mut moves = jets.chars().enumerate().cycle();

    let mut cy = 0;
    let mut cx = 0;
    let mut hy = 0;
    let mut hx = 0;

    for (c,&piece) in PIECES.iter().cycle().enumerate() {
        if c == piece_count {
            println!("After {c} rocks, the tower will be {} tall", tetris.height());
        }

        let (im, mut m) = moves.next().unwrap();
        if c % PIECES.len() == 0 && im == 7 {
            let h = tetris.height();
            hx = h - hy; hy = h;
            cx = c - cy; cy = c;
            println!("Starting sequence after {c}/{cx} rocks, height = {h}/{hx}");
        }
        if cx == 1705 && hx == 2649 && (piece_count_long - c) % cx == 0 {
            println!("Stopping sequence after {c} rocks, height = {}", tetris.height());
            let remaining_height = (piece_count_long - c) / cx * hx;
            println!("After {piece_count_long} rocks, the tower will be {} tall",
                tetris.height() + remaining_height);
            break;
        }

        tetris.add(piece);
        loop {
            tetris.push(m);
            if tetris.fall() {
                break
            }
            m = moves.next().unwrap().1;
        }
    }
}

impl Tetris {
    pub fn new() -> Self {
        Tetris { columns: Default::default(), piece: None, pos: (0,0) }
    }

    pub fn height(&self) -> usize {
        self.columns.iter().map(|c| c.len()).max().unwrap()
    }

    pub fn add(&mut self, piece: &'static [(i64,i64)]) {
        assert!(self.piece.is_none());
        self.piece = Some(piece);
        self.pos = (2, self.height() as i64 + 3);
    }

    pub fn push(&mut self, dir: char) {
        let old_pos = self.pos;
        match dir {
            '<' => self.pos.0 -= 1,
            '>' => self.pos.0 += 1,
            _ => panic!()
        }
        if self.clash() {
            self.pos = old_pos;
        }
    }

    pub fn fall(&mut self) -> bool {
        self.pos.1 -= 1;
        if self.clash() {
            self.pos.1 += 1;
            for b in self.piece.unwrap() {
                let x = usize::try_from(self.pos.0 + b.0).unwrap();
                let y = usize::try_from(self.pos.1 + b.1).unwrap();
                let v = &mut self.columns[x];
                if v.len() < y+1 {
                    v.resize(y+1, false);
                }
                v[y] = true;
            }
            self.piece = None;
            true
        } else {
            false
        }
    }

    fn clash(&self) -> bool {
        for b in self.piece.unwrap() {
            let p = (self.pos.0 + b.0, self.pos.1 + b.1);
            if p.0 < 0 || p.0 > 6 || p.1 < 0 {
                return true;
            }
            if *self.columns[p.0 as usize].get(p.1 as usize).unwrap_or(&false) {
                return true;
            }
        }
        false
    }
}

use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;

struct Tetris {
    columns: [Vec<bool>; 7],
    piece: Option<&'static [(i32,i32)]>,
    pos: (i32,i32),
}
type ProfileType = u32;
type Profile = [ProfileType; 7];

const PIECES: &[&[(i32,i32)]] = &[
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
    let piece_count_long: usize = 1_000_000_000_000;

    let mut tetris = Tetris::new();
    // (piece_index,jet_index) -> (count,height,profile)
    let mut visited = HashMap::<(usize,usize),(usize,usize,Profile)>::new();
    // Option<(piece_count,growth)>
    let mut period: Option<(usize,usize)> = None;

    let mut moves = jets.chars().enumerate().cycle();
    for (count, (piece_index, &piece)) in PIECES.iter().enumerate().cycle().enumerate() {
        if count == piece_count {
            println!("After {count} rocks, the tower will be {} tall", tetris.height());
        }

        let (jet_index, jet) = moves.next().unwrap();
        let height = tetris.height();
        let profile = tetris.profile();

        match period {
            Some((piece_count,growth)) => {
                let remaining_cycles = piece_count_long - count;
                if remaining_cycles % piece_count == 0 {
                    println!("After {piece_count_long} rocks, the tower will be {} tall",
                        height + (remaining_cycles / piece_count) * growth);
                    break;
                }
            }
            None => {
                if let Some((prev_count,prev_height,prev_profile)) = visited.insert((piece_index,jet_index), (count,height,profile)) {
                    if prev_profile == profile {
                        period = Some((count - prev_count, height - prev_height));
                    }
                }
            }
        }

        tetris.add(piece);
        let mut direction = jet;
        loop {
            tetris.push(direction);
            if tetris.fall() {
                break
            }
            direction = moves.next().unwrap().1;
        }

        let new_profile = tetris.profile();
        assert_ne!(profile, new_profile, "Increase the size of ProfileType");
    }
}

impl Tetris {
    pub fn new() -> Self {
        Tetris { columns: Default::default(), piece: None, pos: (0,0) }
    }

    pub fn height(&self) -> usize {
        self.columns.iter().map(|c| c.len()).max().unwrap()
    }

    pub fn add(&mut self, piece: &'static [(i32,i32)]) {
        assert!(self.piece.is_none());
        self.piece = Some(piece);
        self.pos = (2, i32::try_from(self.height()).unwrap() + 3);
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

    pub fn profile(&self) -> Profile {
        let height = self.height() as isize;
        let mut profile: Profile = [0; 7];
        for (v,p) in self.columns.iter().zip(&mut profile) {
            for h in (height - ProfileType::BITS as isize)..height {
                let bit = if h < 0 { true } else { *v.get(h as usize).unwrap_or(&false) };
                *p = (*p << 1) | (bit as ProfileType);
            }
        }
        profile
    }
}

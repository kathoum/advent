use std::fs::File;
use std::io::{BufReader, Read};
use std::iter::repeat_n;

fn main() {
    let mut disk = Vec::new();
    let mut blocks = Vec::new();
    for (i, c) in BufReader::new(File::open("input/day09.txt").unwrap())
        .bytes()
        .enumerate()
    {
        let i = i as i32;
        let c = c.unwrap();
        if c.is_ascii_digit() {
            let n = c - b'0';
            if i % 2 == 0 {
                disk.extend(repeat_n(i / 2, n.into()));
                blocks.push(Block {
                    fileid: i / 2,
                    len: n.into(),
                });
            } else {
                disk.extend(repeat_n(-1, n.into()));
                blocks.push(Block {
                    fileid: -1,
                    len: n.into(),
                });
            }
        }
    }

    defrag(&mut disk);
    println!("Day 9 part one: {}", disk_checksum(&disk));

    compact(&mut blocks);
    println!("Day 9 part one: {}", block_checksum(&blocks));
}

fn defrag(disk: &mut [i32]) {
    let mut i = 0;
    let mut j = disk.len();
    loop {
        let Some(n) = disk[i..].iter().position(|x| *x == -1) else {
            break;
        };
        i += n;
        assert_eq!(disk[i], -1);

        let Some(n) = disk[..j].iter().rev().position(|x| *x != -1) else {
            break;
        };
        j -= n + 1;
        assert_ne!(disk[j], -1);

        if i < j {
            disk.swap(i, j);
        } else {
            break;
        }
    }
}

fn disk_checksum(disk: &[i32]) -> i64 {
    disk.iter()
        .enumerate()
        .map(|(i, &x)| (i as i64) * (x.max(0) as i64))
        .sum()
}

struct Block {
    fileid: i32,
    len: u32,
}

fn compact(blocks: &mut Vec<Block>) {
    let mut i = blocks.len() - 1;
    let mut fileid = blocks[i].fileid;
    assert_ne!(fileid, -1);

    loop {
        assert_eq!(blocks[i].fileid, fileid);
        let len = blocks[i].len;
        if let Some(j) = blocks.iter().position(|b| b.fileid == -1 && b.len >= len) {
            if j < i {
                let leftover = blocks[j].len - len;

                blocks[j].fileid = fileid;
                blocks[j].len = len;

                if leftover > 0 {
                    blocks.insert(
                        j + 1,
                        Block {
                            fileid: -1,
                            len: leftover,
                        },
                    );
                    i += 1;
                }

                blocks[i].fileid = -1;

                if let Some(next) = blocks.get(i + 1) {
                    if next.fileid == -1 {
                        blocks[i].len += next.len;
                        blocks.remove(i + 1);
                    }
                }

                if let Some(prev) = blocks.get(i - 1) {
                    if prev.fileid == -1 {
                        blocks[i].len += prev.len;
                        blocks.remove(i - 1);
                        i -= 1;
                    }
                }
            }
        }

        fileid -= 1;
        let Some(j) = blocks[..i].iter().rev().position(|b| b.fileid == fileid) else {
            return;
        };
        i -= j + 1;
    }
}

fn block_checksum(blocks: &[Block]) -> i64 {
    let mut pos = 0;
    let mut chk = 0;
    for block in blocks {
        let len = block.len as i64;
        if block.fileid != -1 {
            chk += (block.fileid as i64) * (pos + pos + len - 1) * len / 2;
        }
        pos += len;
    }
    chk
}

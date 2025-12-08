use std::cell::RefCell;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

fn main() {
    let boxes: Vec<[i64; 3]> = BufReader::new(File::open("input/day08.txt").unwrap())
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let mut i = line.split(',');
            let x = i.next().unwrap().parse().unwrap();
            let y = i.next().unwrap().parse().unwrap();
            let z = i.next().unwrap().parse().unwrap();
            assert!(i.next().is_none());
            [x, y, z]
        })
        .collect();

    let mut distances = BinaryHeap::new();
    for (i, bi) in boxes.iter().enumerate() {
        for (j, bj) in boxes[..i].iter().enumerate() {
            let d = (bi[0] - bj[0]).pow(2) + (bi[1] - bj[1]).pow(2) + (bi[2] - bj[2]).pow(2);
            distances.push((Reverse(d), i, j));
        }
    }

    let mut circuits: Vec<Rc<RefCell<Vec<usize>>>> = boxes
        .iter()
        .enumerate()
        .map(|(i, _)| Rc::new(RefCell::new(vec![i])))
        .collect();

    for _ in 0..1000 {
        let (_, i, j) = distances.pop().unwrap();
        merge_circuits(&mut circuits, i, j);
    }

    let mut sorted_circuits: Vec<Vec<usize>> =
        circuits.iter().map(|c| c.borrow().clone()).collect();
    sorted_circuits.sort();
    sorted_circuits.dedup();
    sorted_circuits.sort_by_key(|c| boxes.len() - c.len());
    println!(
        "Day 8 part one: {}",
        sorted_circuits[0].len() * sorted_circuits[1].len() * sorted_circuits[2].len()
    );

    let (i, j) = loop {
        let (_, i, j) = distances.pop().unwrap();
        merge_circuits(&mut circuits, i, j);
        if circuits[0].borrow().len() == boxes.len() {
            break (i, j);
        }
    };
    println!("Day 8 part two: {}", boxes[i][0] * boxes[j][0]);
}

fn merge_circuits(circuits: &mut [Rc<RefCell<Vec<usize>>>], i: usize, j: usize) {
    let ci = Rc::clone(&circuits[i]);
    let cj = Rc::clone(&circuits[j]);
    if !Rc::ptr_eq(&ci, &cj) {
        for &k in ci.borrow().iter() {
            circuits[k] = cj.clone();
        }
        cj.borrow_mut().append(&mut ci.borrow_mut());
    }
}

use std::collections::VecDeque;

fn main() {
    let input = include_str!("input22.txt");
    let mut line = input.lines();
    
    let mut deck1 = VecDeque::new();
    assert_eq!(line.next(), Some("Player 1:"));
    while let Ok(n) = line.next().unwrap_or("").parse::<i32>() {
        deck1.push_back(n);
    }
    let original_deck1 = deck1;

    let mut deck2 = VecDeque::new();
    assert_eq!(line.next(), Some("Player 2:"));
    while let Ok(n) = line.next().unwrap_or("").parse::<i32>() {
        deck2.push_back(n);
    }
    let original_deck2 = deck2;

    println!("Part One");
    let mut deck1 = original_deck1.clone();
    let mut deck2 = original_deck2.clone();
    while !deck1.is_empty() && !deck2.is_empty() {
        let n1 = deck1.pop_front().unwrap();
        let n2 = deck2.pop_front().unwrap();
        if n1 > n2 {
            deck1.push_back(n1);
            deck1.push_back(n2);
        } else {
            deck2.push_back(n2);
            deck2.push_back(n1);
        }
    }
    println!("Player {} won with score: {}",
        if deck1.is_empty() { 2 } else { 1 },
        deck_score(&deck1) + deck_score(&deck2));

    println!("Part Two");
    let (winner, winner_deck) = play_recursive_game(original_deck1, original_deck2);
    println!("Player {} won with score: {}",
        match winner { Winner::Player1 => 1, Winner::Player2 => 2 },
        deck_score(&winner_deck));
}

fn deck_score(deck: &VecDeque<i32>) -> i32 {
    deck.iter().rev()
        .zip(1..).map(|(n, i)| n * i)
        .sum()
}

enum Winner { Player1, Player2 }
fn play_recursive_game(mut deck1: VecDeque<i32>, mut deck2: VecDeque<i32>) -> (Winner, VecDeque<i32>) {
    let mut played_positions: Vec<(VecDeque<i32>, VecDeque<i32>)> = Vec::new();
    loop {
        let record = (deck1.clone(), deck2.clone());
        if played_positions.contains(&record) {
            return (Winner::Player1, deck1)
        }
        played_positions.push(record);

        if deck1.is_empty() {
            return (Winner::Player2, deck2)
        }
        if deck2.is_empty() {
            return (Winner::Player1, deck1)
        }

        let n1 = deck1.pop_front().unwrap();
        let n2 = deck2.pop_front().unwrap();
        let winner = if deck1.len() as i32 >= n1 && deck2.len() as i32 >= n2 {
            let mut subgame1 = deck1.clone();
            let mut subgame2 = deck2.clone();
            subgame1.truncate(n1 as usize);
            subgame2.truncate(n2 as usize);
            play_recursive_game(subgame1, subgame2).0
        } else if n1 > n2 {
            Winner::Player1
        } else {
            Winner::Player2
        };

        match winner {
            Winner::Player1 => { deck1.push_back(n1); deck1.push_back(n2); }
            Winner::Player2 => { deck2.push_back(n2); deck2.push_back(n1); }
        }
    }
}

use std::collections::{HashSet, VecDeque};

fn parse_input(input: &str) -> (VecDeque<u8>, VecDeque<u8>) {
    let mut it = input.split("\n\n");
    let mut player1 = VecDeque::new();
    for line in it.next().unwrap().lines().skip(1) {
        player1.push_back(line.parse().unwrap());
    }

    let mut player2 = VecDeque::new();
    for line in it.next().unwrap().lines().skip(1) {
        player2.push_back(line.parse().unwrap());
    }

    (player1, player2)
}

pub fn part1(input: &str) -> usize {
    let (mut player1, mut player2) = parse_input(input);

    while !(player1.is_empty() || player2.is_empty()) {
        let p1 = player1.pop_front().unwrap();
        let p2 = player2.pop_front().unwrap();

        if p1 > p2 {
            player1.push_back(p1);
            player1.push_back(p2);
        } else if p2 > p1 {
            // what about ties?
            player2.push_back(p2);
            player2.push_back(p1);
        }
    }

    if player1.is_empty() {
        score(player2)
    } else {
        score(player1)
    }
}

fn score(deck: VecDeque<u8>) -> usize {
    deck.into_iter()
        .rev()
        .enumerate()
        .map(|(idx, value)| (idx + 1) * value as usize)
        .sum()
}

enum Winner {
    Player1,
    Player2,
}

fn recursive_combat(
    mut deck_p1: VecDeque<u8>,
    mut deck_p2: VecDeque<u8>,
) -> (Winner, VecDeque<u8>) {
    let mut played_set: HashSet<(VecDeque<u8>, VecDeque<u8>)> = HashSet::new();
    while !(deck_p1.is_empty() || deck_p2.is_empty()) {
        // check if we played this game yet, set insertion
        // returns false if set already contained element
        if !played_set.insert((deck_p1.clone(), deck_p2.clone())) {
            return (Winner::Player1, deck_p1);
        }

        let p1_card = deck_p1.pop_front().unwrap();
        let p2_card = deck_p2.pop_front().unwrap();

        let winner = if deck_p1.len() >= p1_card as usize && deck_p2.len() >= p2_card as usize {
            let new_deck_p1 = deck_p1.iter().take(p1_card as usize).cloned().collect();
            let new_deck_p2 = deck_p2.iter().take(p2_card as usize).cloned().collect();
            recursive_combat(new_deck_p1, new_deck_p2).0
        } else {
            if p1_card > p2_card {
                Winner::Player1
            } else if p2_card > p1_card {
                Winner::Player2
            } else {
                // the tie is a lie
                continue;
            }
        };

        match winner {
            Winner::Player1 => {
                deck_p1.push_back(p1_card);
                deck_p1.push_back(p2_card);
            }
            Winner::Player2 => {
                deck_p2.push_back(p2_card);
                deck_p2.push_back(p1_card);
            }
        };
    }

    if deck_p1.is_empty() {
        (Winner::Player2, deck_p2)
    } else {
        (Winner::Player1, deck_p1)
    }
}

pub fn part2(input: &str) -> usize {
    let (player1, player2) = parse_input(input);

    let (_, deck) = recursive_combat(player1, player2);

    score(deck)
}

#[cfg(test)]
mod test {
    #[test]
    fn part1() {
        let input = include_str!("input");
        assert_eq!(super::part1(input), 32629);
    }

    #[test]
    fn part2() {
        let input = include_str!("input");
        assert_eq!(super::part2(input), 32519);
    }
}

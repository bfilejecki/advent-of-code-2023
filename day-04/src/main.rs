use std::collections::HashMap;
use std::{collections::HashSet, fs::read_to_string};

fn main() {
    let input = read_to_string("./input.txt").expect("File should exist");
    // let test_input = read_to_string("./test-input.txt").expect("File should exist");
    println!("Part one result is {}", part_one(&input));
    println!("Part two result is {}", part_two(&input));
}

fn part_one(input: &str) -> u32 {
    let mut score = 0;
    for line in input.lines() {
        let card = parse_card(line);
        score = score + score_card(&card);
    }

    return score;
}

fn part_two(input: &str) -> u32 {
    let mut cards: Vec<Card> = Vec::new();
    let mut card_map: HashMap<u32, u32> = HashMap::new();

    for line in input.lines() {
        let card = parse_card(line);
        cards.push(card);
    }

    for card in cards.iter() {
        card_map.insert(card.id.clone() as u32, 1);
    }

    for card in cards.iter() {
        let score = count_hits_for_card(card);
        let count = card_map.get(&(card.id as u32)).unwrap().clone();

        for i in ((card.id as u32) + 1)..(card.id as u32 + score + 1) {
            if let Some(c) = card_map.get_mut(&i) {
                *c = *c + count;
            }
        }
    }
    return card_map.values().sum();
}

fn score_card(card: &Card) -> u32 {
    let hit_count = card
        .numbers
        .iter()
        .filter(|n| card.winning_numbers.contains(n))
        .count();

    if hit_count == 0 {
        return 0;
    }
    return 2u32.pow(hit_count as u32 - 1);
}

fn count_hits_for_card(card: &Card) -> u32 {
    return card
        .numbers
        .iter()
        .filter(|n| card.winning_numbers.contains(n))
        .count() as u32;
}

fn parse_card(raw_card: &str) -> Card {
    let card_id = parse_card_id(raw_card);
    let winning_numbers = parse_winning_numbers(raw_card);
    let numbers = parse_numbers(raw_card);
    return Card {
        id: card_id,
        winning_numbers,
        numbers,
    };
}

struct Card {
    id: u8,
    winning_numbers: HashSet<u8>,
    numbers: HashSet<u8>,
}

fn parse_card_id(raw_card: &str) -> u8 {
    return raw_card
        .split_once(':')
        .unwrap()
        .0
        .split_once(" ")
        .unwrap()
        .1
        .trim()
        .parse::<u8>()
        .unwrap();
}

fn parse_winning_numbers(raw_card: &str) -> HashSet<u8> {
    return raw_card
        .split_once(':')
        .unwrap()
        .1
        .split_once('|')
        .unwrap()
        .0
        .split(' ')
        .map(|n| n.parse::<u8>())
        .filter(|n| n.is_ok())
        .map(|n| n.unwrap())
        .collect();
}

fn parse_numbers(raw_card: &str) -> HashSet<u8> {
    return raw_card
        .split_once(':')
        .unwrap()
        .1
        .split_once('|')
        .unwrap()
        .1
        .split(' ')
        .map(|n| n.parse::<u8>())
        .filter(|n| n.is_ok())
        .map(|n| n.unwrap())
        .collect();
}

#[cfg(test)]
mod tests {
    use crate::parse_card;
    use crate::score_card;
    use crate::HashSet;

    #[test]
    fn test_parse_card() {
        // Given
        let raw_card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected_winning_numbers: HashSet<u8> =
            [41u8, 48u8, 83u8, 86u8, 17u8].into_iter().collect();
        let expected_numbers: HashSet<u8> = [83u8, 86u8, 6u8, 31u8, 17u8, 9u8, 48u8, 53u8]
            .into_iter()
            .collect();

        // When
        let result = parse_card(raw_card);

        // Then
        assert_eq!(result.id, 1);
        assert_eq!(result.winning_numbers, expected_winning_numbers);
        assert_eq!(result.numbers, expected_numbers);
    }

    #[test]
    fn test_score_card() {
        // Given
        let raw_card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let card = parse_card(raw_card);

        // When
        let result = score_card(&card);

        // Then
        assert_eq!(result, 8);
    }
}

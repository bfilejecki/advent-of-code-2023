use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    str::FromStr,
};

fn main() {
    let input = read_to_string("./input.txt").expect("File should exist");
    // let input = read_to_string("./input-50.txt").expect("File should exist");
    // let input = read_to_string("./test-input.txt").expect("File should exist");
    println!("Part one result is {}", part_one(&input));
    println!("Part two result is {}", part_two(&input));
}

fn part_one(input: &str) -> u32 {
    let mut hand_list: Vec<Hand> = input.lines().map(|l| l.parse::<Hand>().unwrap()).collect();
    hand_list.sort();
    hand_list.iter().for_each(|h| {
        dbg!(h);
    });
    return hand_list
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid * (i as u32 + 1))
        .sum();
}

fn part_two(input: &str) -> u64 {
    let mut hand_list: Vec<Hand2> = input.lines().map(|l| l.parse::<Hand2>().unwrap()).collect();
    hand_list.sort();
    hand_list.iter().for_each(|h| {
        dbg!(h);
    });
    println!("Size of hand_list {}", hand_list.len());
    return hand_list
        .iter()
        .enumerate()
        .map(|(i, h)| h.bid as u64 * (i as u64 + 1))
        .sum();
}

#[derive(Eq, Debug)]
struct Hand2 {
    cards: Vec<Card2>,
    bid: u32,
}

#[derive(Eq, Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn hand_type(&self) -> Option<HandType> {
        if self.cards.len() < 5 {
            return None;
        }

        let mut set = HashSet::new();
        let mut map: HashMap<&Card, u8> = HashMap::new();
        self.cards.iter().for_each(|c| {
            set.insert(c);
            if let Some(value) = map.get_mut(c) {
                *value = *value + 1;
            } else {
                map.insert(c, 1);
            };
        });

        return match set.len() {
            1 => Some(HandType::FiveOfAKind),
            2 => match map.values().max() {
                Some(count) => match count {
                    4 => Some(HandType::FourOfAKind),
                    3 => Some(HandType::FullHouse),
                    _ => None,
                },
                None => None,
            },
            3 => match map.values().max() {
                Some(count) => match count {
                    3 => Some(HandType::ThreeOfAKind),
                    2 => Some(HandType::TwoPairs),
                    _ => None,
                },
                None => None,
            },
            4 => Some(HandType::Pair),
            5 => Some(HandType::HighCard),
            _ => None,
        };
    }
}

impl Hand2 {
    fn hand_type(&self) -> Option<HandType> {
        if self.cards.len() < 5 {
            return None;
        }

        let mut set = HashSet::new();
        let mut map: HashMap<&Card2, u8> = HashMap::new();
        self.cards.iter().for_each(|c| {
            set.insert(c);
            if let Some(value) = map.get_mut(c) {
                *value = *value + 1;
            } else {
                map.insert(c, 1);
            };
        });

        let hand_type = match set.len() {
            1 => Some(HandType::FiveOfAKind),
            2 => match map.values().max() {
                Some(count) => match count {
                    4 => Some(HandType::FourOfAKind),
                    3 => Some(HandType::FullHouse),
                    _ => None,
                },
                None => None,
            },
            3 => match map.values().max() {
                Some(count) => match count {
                    3 => Some(HandType::ThreeOfAKind),
                    2 => Some(HandType::TwoPairs),
                    _ => None,
                },
                None => None,
            },
            4 => Some(HandType::Pair),
            5 => Some(HandType::HighCard),
            _ => None,
        };

        let j_count = if let Some(j) = map.get(&Card2::J) {
            j
        } else {
            return hand_type;
        };

        return match hand_type {
            Some(ht) => match ht {
                HandType::FiveOfAKind => Some(HandType::FiveOfAKind),
                HandType::FourOfAKind => Some(HandType::FiveOfAKind),
                HandType::ThreeOfAKind => Some(HandType::FourOfAKind),
                HandType::FullHouse => Some(HandType::FiveOfAKind),
                HandType::TwoPairs => {
                    if *j_count == 2 as u8 {
                        Some(HandType::FourOfAKind)
                    } else {
                        Some(HandType::FullHouse)
                    }
                }
                HandType::Pair => Some(HandType::ThreeOfAKind),
                HandType::HighCard => Some(HandType::Pair),
            },
            None => None,
        };
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type().unwrap().cmp(&other.hand_type().unwrap()) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => {
                let mut res = std::cmp::Ordering::Equal;
                for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                    let tmp_res = c1.cmp(c2);
                    if tmp_res != res {
                        println!("Comparing cards");
                        dbg!(c1, c2, tmp_res);
                        res = tmp_res;
                        break;
                    }
                }

                res
            }
        }
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type().unwrap().cmp(&other.hand_type().unwrap()) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => {
                let mut res = std::cmp::Ordering::Equal;
                for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                    let tmp_res = c1.cmp(c2);
                    if tmp_res != res {
                        println!("Comparing cards");
                        dbg!(c1, c2, tmp_res);
                        res = tmp_res;
                        break;
                    }
                }

                res
            }
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type().unwrap().cmp(&other.hand_type().unwrap()) {
            std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
            std::cmp::Ordering::Equal => {
                let mut res = std::cmp::Ordering::Equal;
                for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                    let tmp_res = c1.cmp(c2);
                    if tmp_res != res {
                        println!("Comparing cards");
                        dbg!(c1, c2, tmp_res);
                        res = tmp_res;
                        break;
                    }
                }
                Some(res)
            }
        }
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type().unwrap().cmp(&other.hand_type().unwrap()) {
            std::cmp::Ordering::Less => Some(std::cmp::Ordering::Less),
            std::cmp::Ordering::Greater => Some(std::cmp::Ordering::Greater),
            std::cmp::Ordering::Equal => {
                let mut res = std::cmp::Ordering::Equal;
                for (c1, c2) in self.cards.iter().zip(other.cards.iter()) {
                    let tmp_res = c1.cmp(c2);
                    if tmp_res != res {
                        println!("Comparing cards");
                        dbg!(c1, c2, tmp_res);
                        res = tmp_res;
                        break;
                    }
                }
                Some(res)
            }
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bid == other.bid
    }
}

impl PartialEq for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bid == other.bid
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eigth,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
enum Card2 {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eigth,
    Nine,
    T,
    Q,
    K,
    A,
}

#[derive(Debug, PartialEq, Eq)]
struct CardParseError;

#[derive(Debug, PartialEq, Eq)]
struct HandParseError;

impl FromStr for Card {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "2" => Ok(Card::Two),
            "3" => Ok(Card::Three),
            "4" => Ok(Card::Four),
            "5" => Ok(Card::Five),
            "6" => Ok(Card::Six),
            "7" => Ok(Card::Seven),
            "8" => Ok(Card::Eigth),
            "9" => Ok(Card::Nine),
            "T" => Ok(Card::T),
            "J" => Ok(Card::J),
            "Q" => Ok(Card::Q),
            "K" => Ok(Card::K),
            "A" => Ok(Card::A),
            _ => Err(CardParseError),
        };
    }
}

impl FromStr for Card2 {
    type Err = CardParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s {
            "2" => Ok(Card2::Two),
            "3" => Ok(Card2::Three),
            "4" => Ok(Card2::Four),
            "5" => Ok(Card2::Five),
            "6" => Ok(Card2::Six),
            "7" => Ok(Card2::Seven),
            "8" => Ok(Card2::Eigth),
            "9" => Ok(Card2::Nine),
            "T" => Ok(Card2::T),
            "J" => Ok(Card2::J),
            "Q" => Ok(Card2::Q),
            "K" => Ok(Card2::K),
            "A" => Ok(Card2::A),
            _ => Err(CardParseError),
        };
    }
}

impl FromStr for Hand {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_str, bid_str) = s.split_once(' ').ok_or(HandParseError)?;

        let mut cards: Vec<Card> = Vec::with_capacity(5);
        cards_str
            .chars()
            .into_iter()
            .take(5)
            .map(|c| c.to_string().parse::<Card>())
            .map(|r| r.map_err(|_| HandParseError))
            .enumerate()
            .for_each(|(i, r)| match r {
                Ok(c) => {
                    cards.push(c);
                }
                Err(e) => {
                    panic!("HandParseError has been received");
                }
            });

        let bid = bid_str.parse::<u32>().map_err(|_| HandParseError)?;

        return Ok(Hand { cards, bid });
    }
}

impl FromStr for Hand2 {
    type Err = HandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards_str, bid_str) = s.split_once(' ').ok_or(HandParseError)?;

        let mut cards: Vec<Card2> = Vec::with_capacity(5);
        cards_str
            .chars()
            .into_iter()
            .take(5)
            .map(|c| c.to_string().parse::<Card2>())
            .map(|r| r.map_err(|_| HandParseError))
            .enumerate()
            .for_each(|(i, r)| match r {
                Ok(c) => {
                    cards.push(c);
                }
                Err(e) => {
                    panic!("HandParseError has been received");
                }
            });

        let bid = bid_str.parse::<u32>().map_err(|_| HandParseError)?;

        return Ok(Hand2 { cards, bid });
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    Pair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[cfg(test)]
mod tests {
    use std::{collections::HashSet, fs::read_to_string};

    use crate::{Card, Hand, HandType};

    #[test]
    fn test_compare_cards() {
        // Given
        let two = Card::Two;
        let three = Card::Three;

        // When
        let result = two < three;

        // Then
        assert_eq!(result, true);
    }

    #[test]
    fn test_enum_hashset() {
        // Given
        let mut set = HashSet::new();

        // When
        set.insert(Card::Two);
        set.insert(Card::Two);
        set.insert(Card::Three);

        // Then
        assert_eq!(set.len(), 2);
    }

    #[test]
    fn test_hand_type() {
        // Given
        let five_of_a_kind: [Card; 5] = [
            Card::Three,
            Card::Three,
            Card::Three,
            Card::Three,
            Card::Three,
        ];
        let four_of_a_kind: [Card; 5] = [
            Card::Two,
            Card::Three,
            Card::Three,
            Card::Three,
            Card::Three,
        ];
        let three_of_a_kind: [Card; 5] =
            [Card::Two, Card::Four, Card::Three, Card::Three, Card::Three];
        let full_house: [Card; 5] = [Card::Two, Card::Two, Card::Three, Card::Three, Card::Three];
        let two_pairs: [Card; 5] = [Card::Two, Card::Two, Card::Three, Card::Three, Card::Five];
        let pair: [Card; 5] = [Card::Two, Card::Two, Card::Six, Card::A, Card::Five];
        let high_card: [Card; 5] = [Card::Two, Card::Three, Card::Six, Card::A, Card::Five];

        // When
        let res_5 = (&Hand {
            cards: five_of_a_kind,
        })
            .hand_type();
        let res_4 = (&Hand {
            cards: four_of_a_kind,
        })
            .hand_type();
        let res_3 = (&Hand {
            cards: three_of_a_kind,
        })
            .hand_type();
        let res_22 = (&Hand { cards: two_pairs }).hand_type();
        let res_2 = (&Hand { cards: pair }).hand_type();
        let res_full = (&Hand { cards: full_house }).hand_type();
        let res_high = (&Hand { cards: high_card }).hand_type();

        // Then
        assert_eq!(res_5.unwrap(), HandType::FiveOfAKind);
        assert_eq!(res_4.unwrap(), HandType::FourOfAKind);
        assert_eq!(res_3.unwrap(), HandType::ThreeOfAKind);
        assert_eq!(res_22.unwrap(), HandType::TwoPairs);
        assert_eq!(res_2.unwrap(), HandType::Pair);
        assert_eq!(res_full.unwrap(), HandType::FullHouse);
        assert_eq!(res_high.unwrap(), HandType::HighCard);
    }
}

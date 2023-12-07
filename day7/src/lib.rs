#[derive(PartialEq, Eq, Debug, PartialOrd, Ord)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    J,
    Queen,
    King,
    Ace,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::Ten,
            'J' => Self::J,
            'Q' => Self::Queen,
            'K' => Self::King,
            'A' => Self::Ace,
            _ => panic!("Invalid card: {}", value),
        }
    }
}

impl Card {
    fn compare(&self, other: &Self, joker: bool) -> std::cmp::Ordering {
        if joker {
            if *self == Card::J && *other == Card::J {
                return std::cmp::Ordering::Equal;
            } else if *self == Card::J {
                return std::cmp::Ordering::Less;
            } else if *other == Card::J {
                return std::cmp::Ordering::Greater;
            }
        }

        if *self > *other {
            return std::cmp::Ordering::Greater;
        } else if *self < *other {
            return std::cmp::Ordering::Less;
        }

        std::cmp::Ordering::Equal
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum HandKind {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandKind {
    fn from_str(value: &str, joker: bool) -> Self {
        let mut counts = std::collections::HashMap::new();

        for c in value.chars() {
            *counts.entry(c).or_insert(0) += 1;
        }

        if joker {
            let amount_of_jokers = counts.remove(&'J').unwrap_or_default();

            let max_char = counts
                .iter()
                .max_by_key(|(_, &count)| count)
                .map(|(c, _)| *c)
                .unwrap_or_default();

            *counts.entry(max_char).or_insert(0) += amount_of_jokers;
        }

        let max_count = counts.values().max().cloned().unwrap_or_default();
        let num_different_cards = counts.len();

        match (max_count, num_different_cards) {
            (5, 1) => return HandKind::FiveOfAKind,
            (4, 2) => return HandKind::FourOfAKind,
            (3, 3) => return HandKind::ThreeOfAKind,
            (2, 3) => return HandKind::TwoPairs,
            (3, 2) => return HandKind::FullHouse,
            (2, 4) => return HandKind::OnePair,
            (1, 5) => return HandKind::HighCard,
            _ => panic!("Invalid hand: {}", value),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Hand {
    kind: HandKind,
    cards: Vec<Card>,
}

impl Hand {
    fn from_str(value: &str, joker: bool) -> Self {
        Hand {
            kind: HandKind::from_str(value, joker),
            cards: value.chars().map(|c| Card::try_from(c).unwrap()).collect(),
        }
    }

    fn compare(&self, other: &Self, joker: bool) -> std::cmp::Ordering {
        if self.kind > other.kind {
            return std::cmp::Ordering::Greater;
        } else if self.kind < other.kind {
            return std::cmp::Ordering::Less;
        } else {
            for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                let cmp = a.compare(b, joker);
                if cmp != std::cmp::Ordering::Equal {
                    return cmp;
                }
            }
        }

        std::cmp::Ordering::Equal
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Play {
    hand: Hand,
    bid: usize,
}

impl Play {
    fn from_str(value: &str, joker: bool) -> Self {
        let (hand, bid) = value.split_once(" ").unwrap();
        Play {
            hand: Hand::from_str(hand, joker),
            bid: bid.parse().unwrap(),
        }
    }

    fn compare(&self, other: &Self, joker: bool) -> std::cmp::Ordering {
        self.hand.compare(&other.hand, joker)
    }
}

fn solver(file_content: &str, joker: bool) -> usize {
    let mut plays = file_content
        .lines()
        .map(|line| Play::from_str(line, joker))
        .collect::<Vec<_>>();

    plays.sort_by(|a, b| a.compare(b, joker));

    plays
        .into_iter()
        .enumerate()
        .map(|(i, p)| p.bid * (i + 1))
        .sum()
}

pub fn solve_part_1(file_content: &str) -> usize {
    solver(file_content, false)
}

pub fn solve_part_2(file_content: &str) -> usize {
    solver(file_content, true)
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE_DATA: &str = r#"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483"#;

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part_1(SAMPLE_DATA), 6440);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part_2(SAMPLE_DATA), 5905);
    }
}

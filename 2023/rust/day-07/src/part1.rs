use crate::errors::AocError;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum Card {
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl std::fmt::Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Card::Jack => write!(f, "J"),
            Card::Number(10) => write!(f, "T"),
            Card::Number(n) => write!(f, "{}", n),
            Card::Queen => write!(f, "Q"),
            Card::King => write!(f, "K"),
            Card::Ace => write!(f, "A"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_cards(cards: &Vec<Card>) -> HandType {
        let mut counter = [0_usize; 13];
        for card in cards {
            let idx = match card {
                Card::Ace => 12,
                Card::King => 11,
                Card::Queen => 10,
                Card::Jack => 9,
                Card::Number(n) => *n as usize - 2,
            };
            counter[idx] += 1;
        }
        match counter.iter().max().unwrap() {
            5 => HandType::FiveOfAKind,
            4 => HandType::FourOfAKind,
            3 => {
                if counter.contains(&2) {
                    HandType::FullHouse
                } else {
                    HandType::ThreeOfAKind
                }
            }
            2 => {
                if counter.iter().filter(|&&n| n == 2).count() == 2 {
                    HandType::TwoPairs
                } else {
                    HandType::OnePair
                }
            }
            1 => HandType::HighCard,
            _ => unreachable!("Invalid card count"),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: u32,
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = self
            .cards
            .iter()
            .map(|card| card.to_string())
            .collect::<Vec<_>>()
            .join("");
        write!(f, "{:12}", format!("{:?}", self.hand_type))?;
        write!(f, " {}", cards)?;
        write!(f, " {:4}", self.bid)
    }
}

impl Hand {
    fn from_str(line: &str) -> Hand {
        let mut line = line.split_whitespace();
        let cards = line
            .next()
            .unwrap()
            .chars()
            .map(|ch| match ch {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'J' => Card::Jack,
                'T' => Card::Number(10),
                _ => Card::Number(ch.to_digit(10).expect("Invalid card") as u8),
            })
            .collect::<Vec<_>>();
        let bid = line.next().unwrap().parse::<u32>().expect("Invalid bid");
        let hand_type = HandType::from_cards(&cards);
        Hand {
            cards,
            hand_type,
            bid,
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => self.cards.cmp(&other.cards),
            other => other,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<u32, AocError> {
    let mut hands = input.lines().map(Hand::from_str).collect::<Vec<_>>();
    hands.sort_unstable();

    let result = hands
        .iter()
        .enumerate()
        // .inspect(|(r, h)| {
        //     println!("{:4} {} - {}", r + 1, h, h.bid * (*r as u32 + 1));
        // })
        .map(|(rank, hand)| hand.bid * (rank as u32 + 1))
        .sum::<u32>();

    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_ord() {
        let mut cards = vec![
            Card::Ace,
            Card::King,
            Card::Number(4),
            Card::Queen,
            Card::Jack,
            Card::Number(10),
            Card::Number(2),
            Card::Number(8),
            Card::Number(7),
            Card::Number(3),
            Card::Number(9),
            Card::Number(5),
            Card::Number(6),
        ];
        cards.sort_unstable();
        assert_eq!(
            cards,
            vec![
                Card::Number(2),
                Card::Number(3),
                Card::Number(4),
                Card::Number(5),
                Card::Number(6),
                Card::Number(7),
                Card::Number(8),
                Card::Number(9),
                Card::Number(10),
                Card::Jack,
                Card::Queen,
                Card::King,
                Card::Ace,
            ]
        );
    }

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(6440, process(input)?);
        Ok(())
    }
}


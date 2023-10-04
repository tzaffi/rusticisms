use itertools::sorted;
use std::cmp::Ordering;

/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    let hands_st = hands
        .iter()
        .map(|s| Hand::from_str(s).unwrap())
        .collect::<Vec<_>>();

    let max_hand = hands_st.iter().max().unwrap();

    let indices = hands_st
        .iter()
        .enumerate()
        .filter(|(_, hand)| hand == &max_hand)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    indices.iter().map(|&i| hands[i]).collect()
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Suit {
    Hearts = 0,
    Diamonds,
    Clubs,
    Spades,
}

impl Suit {
    /// Given a single-character string, return the corresponding suit.
    ///
    /// If the string is not a valid suit, return `None`.
    pub fn from_str(s: &str) -> Option<Suit> {
        match s {
            "H" => Some(Suit::Hearts),
            "D" => Some(Suit::Diamonds),
            "C" => Some(Suit::Clubs),
            "S" => Some(Suit::Spades),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank {
    Two = 2,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Rank {
    /// Given a single-character string, return the corresponding rank.
    ///
    /// If the string is not a valid rank, return `None`.
    pub fn from_str(s: &str) -> Option<Rank> {
        match s {
            "2" => Some(Rank::Two),
            "3" => Some(Rank::Three),
            "4" => Some(Rank::Four),
            "5" => Some(Rank::Five),
            "6" => Some(Rank::Six),
            "7" => Some(Rank::Seven),
            "8" => Some(Rank::Eight),
            "9" => Some(Rank::Nine),
            "10" => Some(Rank::Ten),
            "J" => Some(Rank::Jack),
            "Q" => Some(Rank::Queen),
            "K" => Some(Rank::King),
            "A" => Some(Rank::Ace),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    /// Given a string like "2H", return the corresponding card.
    ///
    /// If the string is not a valid card, return `None`.
    pub fn from_str(s: &str) -> Option<Card> {
        let bytes = s.as_bytes();
        let slen = bytes.len();
        if slen < 2 || slen > 3 {
            return None;
        }

        let suit = Suit::from_str(&(bytes[slen - 1] as char).to_string())?;
        if slen == 2 {
            let rank = Rank::from_str(&(bytes[0] as char).to_string())?;
            return Some(Card { rank, suit });
        }

        // WLOG s.len() == 3
        if bytes[0] != '1' as u8 {
            return None;
        }
        if bytes[1] != '0' as u8 {
            return None;
        }
        Some(Card {
            rank: Rank::Ten,
            suit,
        })
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Hand(pub (Card, Card, Card, Card, Card));

impl Hand {
    pub fn new(c1: Card, c2: Card, c3: Card, c4: Card, c5: Card) -> Result<Self, &'static str> {
        if c1 < c2 && c2 < c3 && c3 < c4 && c4 < c5 {
            Ok(Self((c1, c2, c3, c4, c5)))
        } else {
            Err("Cards are not in order")
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        let cards = s
            .split_whitespace()
            .map(|s| Card::from_str(s))
            .collect::<Option<Vec<_>>>()?;
        if cards.len() != 5 {
            return None;
        }
        let scards: Vec<Card> = sorted(cards).collect();

        Self::new(scards[0], scards[1], scards[2], scards[3], scards[4]).ok()
    }

    pub fn is_flush(&self) -> bool {
        let cards = self.0;
        [(cards).0, (cards).1, (cards).2, (cards).3, (cards).4]
            .windows(2)
            .all(|w| w[0].suit == w[1].suit)
    }

    pub fn is_straight(&self) -> bool {
        let cards = self.0;
        [(cards).0, (cards).1, (cards).2, (cards).3, (cards).4]
            .windows(2)
            .all(|w| w[0].rank as u8 + 1 == w[1].rank as u8)
    }

    pub fn kind_groups(&self) -> (KindGroups, Vec<Rank>) {
        let cards = [self.0 .0, self.0 .1, self.0 .2, self.0 .3, self.0 .4];
        let mut groups = vec![];
        let mut remainder = vec![];
        let mut cur_rank = cards[0].rank;
        let mut cur_count = 1;
        for i in 0..4 {
            if cards[i].rank == cards[i + 1].rank {
                cur_count += 1;
            } else {
                if cur_count > 1 {
                    groups.push((cur_rank, cur_count));
                } else {
                    remainder.push(cur_rank);
                }
                cur_rank = cards[i + 1].rank;
                cur_count = 1;
            }
        }
        if cur_count > 1 {
            groups.push((cur_rank, cur_count));
        } else {
            remainder.push(cur_rank);
        }
        groups.sort_by(|a, b| b.1.cmp(&a.1));
        remainder.sort();
        remainder.reverse();
        (KindGroups(groups), remainder)
    }

    pub fn categorize(&self) -> (Category, KindGroups, Vec<Rank>) {
        let (groups, remainders) = self.kind_groups();
        match groups.0.len() {
            0 => {
                if self.is_flush() {
                    if self.is_straight() {
                        return (Category::StraightFlush, groups, remainders);
                    }
                    return (Category::Flush, groups, remainders);
                }
                (Category::HighCard, groups, remainders)
            }
            1 => match groups.0[0].1 {
                2 => (Category::OnePair, groups, remainders),
                3 => (Category::ThreeOfAKind, groups, remainders),
                4 => (Category::FourOfAKind, groups, remainders),
                _ => unreachable!(),
            },
            2 => match groups.0[0].1 {
                2 => (Category::TwoPairs, groups, remainders),
                3 => (Category::FullHouse, groups, remainders),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmp(other).into()
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let (cat1, groups1, remainders1) = self.categorize();
        let (cat2, groups2, remainders2) = other.categorize();
        if cat1 != cat2 {
            return cat1.cmp(&cat2);
        }
        if groups1 != groups2 {
            return groups1.partial_cmp(&groups2).unwrap();
        }
        remainders1.partial_cmp(&remainders2).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, PartialOrd, Ord)]
pub enum Category {
    HighCard = 0,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    Straight,
    Flush,
    FullHouse,
    FourOfAKind,
    StraightFlush,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct KindGroups(pub Vec<(Rank, usize)>);

impl KindGroups {
    pub fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cards = &self.0;
        if cards.len() != other.0.len() {
            return None;
        }
        for i in 0..cards.len() {
            if cards[i].1 != other.0[i].1 {
                return None;
            }
            if cards[i].0 != other.0[i].0 {
                return cards[i].0.partial_cmp(&other.0[i].0);
            }
        }
        Some(Ordering::Equal)
    }
}

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

    let max_hand = match hands_st.iter().max() {
        Some(h) => *h,
        None => return vec![],
    };

    let indices = hands_st
        .iter()
        .enumerate()
        .filter(|(_, hand)| hand.cmp(&&max_hand) == Ordering::Equal)
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
        if s.starts_with("10") {
            return Some(Card {
                rank: Rank::Ten,
                suit,
            });
        }
        None
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

    // Assumes that the cards are sorted descending by rank
    // returns (is_straight, is_low_ace)
    pub fn straight_type(ranks: &Vec<Rank>) -> (bool, bool) {
        fn sub_straight(ranks: &[Rank]) -> bool {
            ranks.windows(2).all(|w| w[0] as u8 - 1 == w[1] as u8)
        }

        if ranks[0] == Rank::Ace {
            if sub_straight(ranks) {
                return (true, false);
            }
            if ranks[1] == Rank::Five && sub_straight(&ranks[1..]) {
                return (true, true);
            }
        }
        (sub_straight(ranks), false)
    }

    /// Calculates and returns the kind groups and remainder cards for a hand.
    ///
    /// The kind groups are represented as a `KindGroups` structure containing
    /// vectors of tuples `(count, rank)`, where `count` is the number of occurrences
    /// of cards with the same rank in the hand, and `rank` is the rank of those cards.
    ///
    /// The remainder is a vector containing the ranks of cards not included in any groups,
    /// sorted in descending order.
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
                    groups.push((cur_count, cur_rank));
                } else {
                    remainder.push(cur_rank);
                }
                cur_rank = cards[i + 1].rank;
                cur_count = 1;
            }
        }
        if cur_count > 1 {
            groups.push((cur_count, cur_rank));
        } else {
            remainder.push(cur_rank);
        }
        groups.sort();
        groups.reverse();
        remainder.sort();
        remainder.reverse();
        (KindGroups(groups), remainder)
    }

    pub fn categorize(&self) -> (Category, KindGroups, Vec<Rank>, bool) {
        // TODO: this should cache the result
        let (groups, remainders) = self.kind_groups();
        match groups.0.len() {
            0 => {
                let (is_straight, is_low_ace) = Self::straight_type(&remainders);
                if self.is_flush() {
                    if is_straight {
                        return (Category::StraightFlush, groups, remainders, is_low_ace);
                    }
                    return (Category::Flush, groups, remainders, is_low_ace);
                }
                if is_straight {
                    return (Category::Straight, groups, remainders, is_low_ace);
                }
                (Category::HighCard, groups, remainders, false)
            }
            1 => match groups.0[0].0 {
                2 => (Category::OnePair, groups, remainders, false),
                3 => (Category::ThreeOfAKind, groups, remainders, false),
                4 => (Category::FourOfAKind, groups, remainders, false),
                _ => unreachable!(),
            },
            2 => match groups.0[0].0 {
                2 => (Category::TwoPairs, groups, remainders, false),
                3 => (Category::FullHouse, groups, remainders, false),
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
        let (cat1, groups1, remainders1, low_ace1) = self.categorize();
        let (cat2, groups2, remainders2, low_ace2) = other.categorize();
        if cat1 != cat2 {
            return cat1.cmp(&cat2);
        }
        if groups1 != groups2 {
            return groups1.partial_cmp(&groups2).unwrap();
        }
        if cat1 == Category::StraightFlush || cat1 == Category::Straight {
            if low_ace1 != low_ace2 {
                return if low_ace1 {
                    Ordering::Less
                } else {
                    Ordering::Greater
                };
            }
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
pub struct KindGroups(pub Vec<(usize, Rank)>);

impl KindGroups {
    pub fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let cards = &self.0;
        if cards.len() != other.0.len() {
            return None;
        }
        for i in 0..cards.len() {
            if cards[i].0 != other.0[i].0 {
                return None;
            }
            if cards[i].1 != other.0[i].1 {
                return cards[i].1.partial_cmp(&other.0[i].1);
            }
        }
        Some(Ordering::Equal)
    }
}

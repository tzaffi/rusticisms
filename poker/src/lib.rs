/// Given a list of poker hands, return a list of those hands which win.
///
/// Note the type signature: this function should return _the same_ reference to
/// the winning hand(s) as were passed in, not reconstructed strings which happen to be equal.
pub fn winning_hands<'a>(hands: &[&'a str]) -> Vec<&'a str> {
    todo!("Out of {hands:?}, which hand wins?")
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
        // let cards = (cards[0], cards[1], cards[2], cards[3], cards[4]);
        Self::new(cards[0], cards[1], cards[2], cards[3], cards[4]).ok()
    }
}

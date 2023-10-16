// use poker::{winning_hands, Card, Category, Hand, KindGroups, Rank, Suit};
use poker::{winning_hands, Card, Hand, Rank, Suit};
// use std::cmp::Ordering;
use std::collections::HashSet;

fn hs_from<'a>(input: &[&'a str]) -> HashSet<&'a str> {
    let mut hs = HashSet::new();
    for item in input.iter() {
        hs.insert(*item);
    }
    hs
}

/// Test that the expected output is produced from the given input
/// using the `winning_hands` function.
///
/// Note that the output can be in any order. Here, we use a HashSet to
/// abstract away the order of outputs.
fn test(input: &[&str], expected: &[&str]) {
    assert_eq!(hs_from(&winning_hands(input)), hs_from(expected))
}

#[test]
fn test_from_str() {
    assert_eq!(Suit::from_str("H"), Some(Suit::Hearts));
    assert_eq!(Suit::from_str("D"), Some(Suit::Diamonds));
    assert_eq!(Suit::from_str("C"), Some(Suit::Clubs));
    assert_eq!(Suit::from_str("S"), Some(Suit::Spades));
    assert_eq!(Suit::from_str("Z"), None);

    assert_eq!(Rank::from_str("2"), Some(Rank::Two));
    assert_eq!(Rank::from_str("3"), Some(Rank::Three));
    assert_eq!(Rank::from_str("4"), Some(Rank::Four));
    assert_eq!(Rank::from_str("5"), Some(Rank::Five));
    assert_eq!(Rank::from_str("6"), Some(Rank::Six));
    assert_eq!(Rank::from_str("7"), Some(Rank::Seven));
    assert_eq!(Rank::from_str("8"), Some(Rank::Eight));
    assert_eq!(Rank::from_str("9"), Some(Rank::Nine));
    assert_eq!(Rank::from_str("10"), Some(Rank::Ten));
    assert_eq!(Rank::from_str("J"), Some(Rank::Jack));
    assert_eq!(Rank::from_str("Q"), Some(Rank::Queen));
    assert_eq!(Rank::from_str("K"), Some(Rank::King));
    assert_eq!(Rank::from_str("A"), Some(Rank::Ace));
    assert_eq!(Rank::from_str("Z"), None);

    assert_eq!(
        Card::from_str("2H"),
        Some(Card {
            rank: Rank::Two,
            suit: Suit::Hearts
        })
    );
    assert_eq! {
        Card::from_str("10D"),
        Some(Card {
            rank: Rank::Ten,
            suit: Suit::Diamonds
        })
    };
    assert_eq! {
        Card::from_str("QH"),
        Some(Card {
            rank: Rank::Queen,
            suit: Suit::Hearts
        })
    };
    assert_eq! {
        Card::from_str("AS"),
        Some(Card {
            rank: Rank::Ace,
            suit: Suit::Spades
        })
    };
    assert_eq!(Card::from_str("1H"), None);
    assert_eq!(Card::from_str("QH2"), None);

    assert_eq!(
        Hand::from_str("2H 3H 4H 5H 6H").unwrap(),
        Hand::new(
            Card {
                rank: Rank::Two,
                suit: Suit::Hearts
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Hearts
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Hearts
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Hearts
            },
            Card {
                rank: Rank::Six,
                suit: Suit::Hearts
            }
        )
        .unwrap()
    );

    assert_eq!(Hand::from_str("bunch of garbage"), None);

    assert_eq!(Hand::from_str("2H 3H 4H 4H 6H"), None);

    assert_eq!(Hand::from_str("2H 3H 4H 5H"), None);

    assert_eq!(Hand::from_str("2H 3H 4H 5H 6H 7H"), None);

    assert_eq!(
        Hand::from_str("2H 3D 4C 5S 10H").unwrap(),
        Hand::new(
            Card {
                rank: Rank::Two,
                suit: Suit::Hearts
            },
            Card {
                rank: Rank::Three,
                suit: Suit::Diamonds
            },
            Card {
                rank: Rank::Four,
                suit: Suit::Clubs
            },
            Card {
                rank: Rank::Five,
                suit: Suit::Spades
            },
            Card {
                rank: Rank::Ten,
                suit: Suit::Hearts
            }
        )
        .unwrap()
    );
}

// #[test]
// fn test_comparisons() {
//     assert_eq!(Suit::Hearts.cmp(&Suit::Diamonds), Ordering::Less);
//     assert_eq!(Suit::Spades.cmp(&Suit::Clubs), Ordering::Greater);
//     assert_eq!(Suit::Diamonds.cmp(&Suit::Diamonds), Ordering::Equal);

//     assert_eq!(Rank::Two.cmp(&Rank::Three), Ordering::Less);
//     assert_eq!(Rank::Ace.cmp(&Rank::King), Ordering::Greater);
//     assert_eq!(Rank::Queen.cmp(&Rank::Queen), Ordering::Equal);

//     assert!(Rank::Two < Rank::Three);
//     assert!(Rank::Ace > Rank::King);
//     assert!(Rank::Queen >= Rank::Queen);
//     assert!(Rank::Queen <= Rank::Queen);

//     assert!(Category::StraightFlush > Category::FourOfAKind);
//     assert!(Category::FourOfAKind > Category::FullHouse);
//     assert!(Category::FullHouse > Category::Flush);
//     assert!(Category::HighCard < Category::OnePair);

//     assert_eq!(
//         KindGroups(vec![(Rank::Two, 4)])
//             .partial_cmp(&KindGroups(vec![(Rank::Three, 4)]))
//             .unwrap(),
//         Ordering::Less
//     );
//     assert_eq!(
//         KindGroups(vec![(Rank::Two, 4)])
//             .partial_cmp(&KindGroups(vec![(Rank::Two, 4)]))
//             .unwrap(),
//         Ordering::Equal
//     );
//     assert_eq!(
//         KindGroups(vec![(Rank::Two, 3)]).partial_cmp(&KindGroups(vec![(Rank::Two, 4)])),
//         None
//     );
//     assert_eq!(
//         KindGroups(vec![(Rank::Two, 3)]).partial_cmp(&KindGroups(vec![])),
//         None
//     );
//     assert_eq!(
//         KindGroups(vec![(Rank::Five, 2), (Rank::Two, 2)])
//             .partial_cmp(&KindGroups(vec![(Rank::Five, 2), (Rank::Two, 2)]))
//             .unwrap(),
//         Ordering::Equal
//     );
//     assert_eq!(
//         KindGroups(vec![(Rank::Five, 2), (Rank::Two, 2)])
//             .partial_cmp(&KindGroups(vec![(Rank::Five, 2), (Rank::Three, 2)]))
//             .unwrap(),
//         Ordering::Less
//     );
//     assert_eq!(
//         KindGroups(vec![(Rank::Six, 2), (Rank::Two, 2)])
//             .partial_cmp(&KindGroups(vec![(Rank::Five, 2), (Rank::Four, 2)]))
//             .unwrap(),
//         Ordering::Greater
//     );
// }

#[test]
fn flush() {
    assert!(Hand::from_str("2H 3H 4H 5H 6H").unwrap().is_flush());
    assert!(Hand::from_str("2H 3H 4H 5H QH").unwrap().is_flush());
    assert!(!Hand::from_str("2H 3H 4H 5S 10H").unwrap().is_flush());
}

// #[test]
// fn kind_groups() {
//     assert_eq!(
//         Hand::from_str("2H 2D 2C 2S 6H").unwrap().kind_groups(),
//         (KindGroups(vec![(Rank::Two, 4)]), vec![Rank::Six])
//     );
//     assert_eq!(
//         Hand::from_str("2H 2D 4H 4C 4S").unwrap().kind_groups(),
//         (KindGroups(vec![(Rank::Four, 3), (Rank::Two, 2)]), vec![])
//     );
//     assert_eq!(
//         Hand::from_str("2H 3D 4H 5C 7S").unwrap().kind_groups(),
//         (
//             KindGroups(vec![]),
//             vec![Rank::Seven, Rank::Five, Rank::Four, Rank::Three, Rank::Two]
//         )
//     );
//     assert_eq!(
//         Hand::from_str("2H 3D 4H 4C 7S").unwrap().kind_groups(),
//         (
//             KindGroups(vec![(Rank::Four, 2)]),
//             vec![Rank::Seven, Rank::Three, Rank::Two]
//         )
//     );
// }

#[test]
fn single_hand_always_wins() {
    test(&["4S 5S 7H 8D JC"], &["4S 5S 7H 8D JC"])
}

#[test]
#[ignore]
fn duplicate_hands_always_tie() {
    let input = &["3S 4S 5D 6H JH", "3S 4S 5D 6H JH", "3S 4S 5D 6H JH"];
    assert_eq!(&winning_hands(input), input)
}

#[test]
#[ignore]
fn highest_card_of_all_hands_wins() {
    test(
        &["4D 5S 6S 8D 3C", "2S 4C 7S 9H 10H", "3S 4S 5D 6H JH"],
        &["3S 4S 5D 6H JH"],
    )
}

#[test]
#[ignore]
fn a_tie_has_multiple_winners() {
    test(
        &[
            "4D 5S 6S 8D 3C",
            "2S 4C 7S 9H 10H",
            "3S 4S 5D 6H JH",
            "3H 4H 5C 6C JD",
        ],
        &["3S 4S 5D 6H JH", "3H 4H 5C 6C JD"],
    )
}

#[test]
#[ignore]
fn high_card_can_be_low_card_in_an_otherwise_tie() {
    // multiple hands with the same high cards, tie compares next highest ranked,
    // down to last card
    test(&["3S 5H 6S 8D 7H", "2S 5D 6D 8C 7S"], &["3S 5H 6S 8D 7H"])
}

#[test]
#[ignore]
fn one_pair_beats_high_card() {
    test(&["4S 5H 6C 8D KH", "2S 4H 6S 4D JH"], &["2S 4H 6S 4D JH"])
}

#[test]
#[ignore]
fn highest_pair_wins() {
    test(&["4S 2H 6S 2D JH", "2S 4H 6C 4D JD"], &["2S 4H 6C 4D JD"])
}

#[test]
#[ignore]
fn two_pairs_beats_one_pair() {
    test(&["2S 8H 6S 8D JH", "4S 5H 4C 8C 5C"], &["4S 5H 4C 8C 5C"])
}

#[test]
#[ignore]
fn two_pair_ranks() {
    // both hands have two pairs, highest ranked pair wins
    test(&["2S 8H 2D 8D 3H", "4S 5H 4C 8S 5D"], &["2S 8H 2D 8D 3H"])
}

#[test]
#[ignore]
fn two_pairs_second_pair_cascade() {
    // both hands have two pairs, with the same highest ranked pair,
    // tie goes to low pair
    test(&["2S QS 2C QD JH", "JD QH JS 8D QC"], &["JD QH JS 8D QC"])
}

#[test]
#[ignore]
fn two_pairs_last_card_cascade() {
    // both hands have two identically ranked pairs,
    // tie goes to remaining card (kicker)
    test(&["JD QH JS 8D QC", "JS QS JC 2D QD"], &["JD QH JS 8D QC"])
}

#[test]
#[ignore]
fn three_of_a_kind_beats_two_pair() {
    test(&["2S 8H 2H 8D JH", "4S 5H 4C 8S 4H"], &["4S 5H 4C 8S 4H"])
}

#[test]
#[ignore]
fn three_of_a_kind_ranks() {
    //both hands have three of a kind, tie goes to highest ranked triplet
    test(&["2S 2H 2C 8D JH", "4S AH AS 8C AD"], &["4S AH AS 8C AD"])
}

#[test]
#[ignore]
fn low_three_of_a_kind_beats_high_two_pair() {
    test(&["2H 2D 2C 8H 5H", "AS AC KS KC 6S"], &["2H 2D 2C 8H 5H"])
}

#[test]
#[ignore]
fn three_of_a_kind_cascade_ranks() {
    // with multiple decks, two players can have same three of a kind,
    // ties go to highest remaining cards
    test(&["4S AH AS 7C AD", "4S AH AS 8C AD"], &["4S AH AS 8C AD"])
}

#[test]
#[ignore]
fn straight_beats_three_of_a_kind() {
    test(&["4S 5H 4C 8D 4H", "3S 4D 2S 6D 5C"], &["3S 4D 2S 6D 5C"])
}

#[test]
#[ignore]
fn aces_can_end_a_straight_high() {
    // aces can end a straight (10 J Q K A)
    test(&["4S 5H 4C 8D 4H", "10D JH QS KD AC"], &["10D JH QS KD AC"])
}

#[test]
#[ignore]
fn aces_can_start_a_straight_low() {
    // aces can start a straight (A 2 3 4 5)
    test(&["4S 5H 4C 8D 4H", "4D AH 3S 2D 5C"], &["4D AH 3S 2D 5C"])
}

#[test]
#[ignore]
fn no_ace_in_middle_of_straight() {
    // aces cannot be in the middle of a straight (Q K A 2 3)
    test(&["2C 3D 7H 5H 2S", "QS KH AC 2D 3S"], &["2C 3D 7H 5H 2S"])
}

#[test]
#[ignore]
fn straight_ranks() {
    // both hands with a straight, tie goes to highest ranked card
    test(&["4S 6C 7S 8D 5H", "5S 7H 8S 9D 6H"], &["5S 7H 8S 9D 6H"])
}

#[test]
#[ignore]
fn straight_scoring() {
    // even though an ace is usually high, a 5-high straight is the lowest-scoring straight
    test(&["2H 3C 4D 5D 6H", "4S AH 3S 2D 5H"], &["2H 3C 4D 5D 6H"])
}

#[test]
#[ignore]
fn flush_beats_a_straight() {
    test(&["4C 6H 7D 8D 5H", "2S 4S 5S 6S 7S"], &["2S 4S 5S 6S 7S"])
}

#[test]
#[ignore]
fn flush_cascade() {
    // both hands have a flush, tie goes to high card, down to the last one if necessary
    test(&["4H 7H 8H 9H 6H", "2S 4S 5S 6S 7S"], &["4H 7H 8H 9H 6H"])
}

#[test]
#[ignore]
fn full_house_beats_a_flush() {
    test(&["3H 6H 7H 8H 5H", "4S 5C 4C 5D 4H"], &["4S 5C 4C 5D 4H"])
}

#[test]
#[ignore]
fn full_house_ranks() {
    // both hands have a full house, tie goes to highest-ranked triplet
    test(&["4H 4S 4D 9S 9D", "5H 5S 5D 8S 8D"], &["5H 5S 5D 8S 8D"])
}

#[test]
#[ignore]
fn full_house_cascade() {
    // with multiple decks, both hands have a full house with the same triplet, tie goes to the pair
    test(&["5H 5S 5D 9S 9D", "5H 5S 5D 8S 8D"], &["5H 5S 5D 9S 9D"])
}

#[test]
#[ignore]
fn four_of_a_kind_beats_full_house() {
    test(&["4S 5H 4D 5D 4H", "3S 3H 2S 3D 3C"], &["3S 3H 2S 3D 3C"])
}

#[test]
#[ignore]
fn four_of_a_kind_ranks() {
    // both hands have four of a kind, tie goes to high quad
    test(&["2S 2H 2C 8D 2D", "4S 5H 5S 5D 5C"], &["4S 5H 5S 5D 5C"])
}

#[test]
#[ignore]
fn four_of_a_kind_cascade() {
    // with multiple decks, both hands with identical four of a kind, tie determined by kicker
    test(&["3S 3H 2S 3D 3C", "3S 3H 4S 3D 3C"], &["3S 3H 4S 3D 3C"])
}

#[test]
#[ignore]
fn straight_flush_beats_four_of_a_kind() {
    test(&["4S 5H 5S 5D 5C", "7S 8S 9S 6S 10S"], &["7S 8S 9S 6S 10S"])
}

#[test]
#[ignore]
fn aces_can_end_a_straight_flush_high() {
    // aces can end a straight flush (10 J Q K A)
    test(&["KC AH AS AD AC", "10C JC QC KC AC"], &["10C JC QC KC AC"])
}

#[test]
#[ignore]
fn aces_can_start_a_straight_flush_low() {
    // aces can start a straight flush (A 2 3 4 5)
    test(&["KS AH AS AD AC", "4H AH 3H 2H 5H"], &["4H AH 3H 2H 5H"])
}

#[test]
#[ignore]
fn no_ace_in_middle_of_straight_flush() {
    // aces cannot be in the middle of a straight flush (Q K A 2 3)
    test(&["2C AC QC 10C KC", "QH KH AH 2H 3H"], &["2C AC QC 10C KC"])
}

#[test]
#[ignore]
fn straight_flush_ranks() {
    // both hands have a straight flush, tie goes to highest-ranked card
    test(&["4H 6H 7H 8H 5H", "5S 7S 8S 9S 6S"], &["5S 7S 8S 9S 6S"])
}

#[test]
#[ignore]
fn straight_flush_scoring() {
    // even though an ace is usually high, a 5-high straight flush is the lowest-scoring straight flush
    test(&["2H 3H 4H 5H 6H", "4D AD 3D 2D 5D"], &["2H 3H 4H 5H 6H"])
}

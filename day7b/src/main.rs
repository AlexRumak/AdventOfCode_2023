use std::{fs, cmp::Ordering, collections::HashMap};

struct Hand<'a> {
    hand: &'a str,
    bid: u64,
}

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should have been able to read input.txt");

    let mut hands: Vec<Hand> = parse_content(&contents);

    sort_hands(&mut hands);

    print_hands(&hands);

    let value = calculate_score(hands);

    println!("Answer: {}", value);
}

fn calculate_score(hands: Vec<Hand<'_>>) -> u64 {
    let mut value = 0;
    for (i, hand) in hands.iter().enumerate() {
        value += hand.bid * (i as u64 + 1);
    }
    value
}

fn sort_hands(hands: &mut Vec<Hand<'_>>) {
    hands.sort_by(|a, b| compare_hand(a, b));
}

fn print_hands(hands: &Vec<Hand>) {
    println!("Printing hands:");
    for hand in hands {
        println!("Hand: {}", hand.hand);
    }
}

fn compare_hand(a: &Hand, b: &Hand) -> Ordering {
    if a.hand == b.hand {
        return Ordering::Equal;
    }

    // Check hand kind
    let a_strength = get_hand_strength(a.hand);
    let b_strength = get_hand_strength(b.hand);

    if a_strength == b_strength {
        // hand strengths are equal, compare element by element
        let mut a_iter = a.hand.chars().peekable();
        let mut b_iter = b.hand.chars().peekable();

        while a_iter.peek().is_some() && b_iter.peek().is_some() {
            let a_char = a_iter.next().expect("Should have a card");
            let b_char = b_iter.next().expect("Should have a card");

            // A > K > Q > J > T > 9 > 8 > 7 > 6 > 5 > 4 > 3 > 2
            let a_value = card_value(a_char);
            let b_value = card_value(b_char);

            if a_value > b_value {
                return Ordering::Greater;
            } else if a_value < b_value {
                return Ordering::Less;
            }
        }

        // Same hand strenght, shouldn't happen in this game
        panic!("Same hand strength - shouldn't happen");
    }
    
    if a_strength > b_strength {
        Ordering::Greater
    } else {
        Ordering::Less
    }
}

fn card_value(a_char: char) -> u32 {
    match a_char {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 1,
        'T' => 10,
        c => c.to_digit(10).expect("Should be number")
    }
}

// Five of a kind 100000
// Four of a kind 10000
// Full house 1000
// Three of a kind 100
// Two pair 10
// One pair 1
// High card 0
fn get_hand_strength(hand: &str) -> u32 {

    // iterate through characters in string
    // TODO: There is probably a collect method for this
    let mut hand_map: HashMap<char, u32>  = HashMap::new();
    for card in hand.chars() {
        if hand_map.contains_key(&card) {
            hand_map.insert(card, hand_map.get(&card).unwrap() + 1);
        } else {
            hand_map.insert(card, 1);
        }
    }

    get_value_from_map(hand_map)
}

fn get_value_from_map(hand_map: HashMap<char, u32>) -> u32 {
    let mut replaced_jack_hand_val = 0;

    if hand_map.contains_key(&'J') {
        let mut removed_jack_map = hand_map.clone();
        let value = removed_jack_map.get(&'J').expect("Should have a J").clone();
        removed_jack_map.remove(&'J');

        for key in removed_jack_map.keys() {
            let mut cloned_map = removed_jack_map.clone();
            cloned_map.insert(*key, cloned_map.get(key).unwrap() + value);
            replaced_jack_hand_val = u32::max(get_value_from_map(cloned_map), replaced_jack_hand_val);
        }
    }

    let mut regular_hand_value = 0;
    if hand_map.len() == 1 {
        regular_hand_value = 100000; // Five of a kind
    }

    if hand_map.len() == 2 {
        if hand_map.values().any(|f| *f == 4) {
            // four of a kind
            regular_hand_value = 10000;
        }
        else {
            // full house
            regular_hand_value = 1000;
        }
    }

    if hand_map.len() == 3 {
        if !hand_map.values().any(|f| *f == 3) {
            // two pair
            regular_hand_value = 10;
        }
        else {
            // three of a kind
            regular_hand_value = 100;
        }
    }

    if hand_map.len() == 4 {
        regular_hand_value = 1; // One pair
    }

    u32::max(regular_hand_value, replaced_jack_hand_val)
    // High card
}

fn parse_content<'a>(contents: &'a String) -> Vec<Hand<'a>> {
    let mut hands: Vec<Hand> = Vec::new();
    for line in contents.lines() {
        let mut split = line.split_whitespace();
        let hand = split.next().expect("Hand should exist");
        let bid = split.next().expect("Bid should exist").parse::<u64>().expect("Bid should be a number");

        hands.push(
            Hand {
                hand: hand,
                bid: bid,
            }
        )
    }

    hands
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_hands_should_work() {
        let mut hands = vec![
            Hand { hand: "KKKK2", bid: 1},
            Hand { hand: "24592", bid: 1},
            Hand { hand: "KK333", bid: 1},
            Hand { hand: "QQT23", bid: 1},
            Hand { hand: "23592", bid: 1},
            Hand { hand: "23591", bid: 1},
            Hand { hand: "55555", bid: 1},
            Hand { hand: "K3591", bid: 1},
            Hand { hand: "KK332", bid: 1},
            Hand { hand: "JJJJJ", bid: 1},
        ];

        sort_hands(&mut hands);

        assert_eq!(hands[0].hand, "23591");
        assert_eq!(hands[1].hand, "K3591");
        assert_eq!(hands[2].hand, "23592");
        assert_eq!(hands[3].hand, "24592");
        assert_eq!(hands[4].hand, "QQT23");
        assert_eq!(hands[5].hand, "KK332");
        assert_eq!(hands[6].hand, "KK333");
        assert_eq!(hands[7].hand, "KKKK2");
        assert_eq!(hands[8].hand, "JJJJJ");
        assert_eq!(hands[9].hand, "55555");


    }

    #[test]
    fn test_calculate_score_should_work() {
        let hands = vec![
            Hand { hand: "23592", bid: 3},
            Hand { hand: "24592", bid: 2},
            Hand { hand: "QQT23", bid: 1},
        ];

        let value = calculate_score(hands);

        assert_eq!(value, 3 + 4 + 3);
    }
}
extern crate rand;
use self::rand::{thread_rng, Rng};

use std::fmt;

#[derive(Clone)]
pub struct Card<'a> {
    suit: &'a char,
    pub card_number: &'a i8
}

impl<'a> fmt::Display for Card<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let card_designation: String = match *self.card_number {
            0 =>  "A".to_string(),
            10 =>  "J".to_string(),
            11 =>  "Q".to_string(),
            12 =>  "K".to_string(),
            x =>  x.to_string()
        };

        let suit_designation: &str = match *self.suit {
            'h' => "♡",
            'd' => "♢",
            'c' => "♣",
            's' => "♠",
            _ => ""
        };
        write!(f, " {}{}", card_designation, suit_designation)
    }
}

pub fn generate<'a>() -> Vec<Card<'a>> {
    static SUITS: [char; 4] = ['h', 'd', 'c', 's'];

    static CARD_RANGE: [i8; 13] = [0,1,2,3,4,5,6,7,8,9,10,11,12];

    let mut cards: Vec<Card> = Vec::new();

    for suit in &SUITS {
        for card_number in &CARD_RANGE {
            cards.push(Card{ suit: suit, card_number: card_number});
        }
    }

    cards
}


pub fn shuffle<'a, 'b>(mut cards: &'b mut Vec<Card<'a>>) -> &'b mut Vec<Card<'a>> {
    let mut rng = thread_rng();
    rng.shuffle(&mut cards);
    cards
}

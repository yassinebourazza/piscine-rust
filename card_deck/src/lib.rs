use rand::prelude::*;

#[derive(Debug)]
pub enum Suit {
    Heart,
    Diamond,
    Spade,
    Club,
}

#[derive(Debug,PartialEq)]
pub enum Rank {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

impl Suit {
    pub fn random() -> Suit {
        let mut generator =  rand::rng();

        let x: u8 = generator.random_range(1..=4);
        Suit::translate(x)
    }

    pub fn translate(value: u8) -> Suit {
        match value {
            1 => Suit::Heart,
            2 => Suit::Diamond,
            3 => Suit::Spade,
            4 => Suit::Club,
            _ => Suit::Club, 
        }
    }
}

impl Rank {
    pub fn random() -> Rank {
        let mut generator = rand::rng();
        let x: u8 = generator.random_range(1..=13);
        Rank::translate(x)
    }

    pub fn translate(value: u8) -> Rank {
         match value {
            1 => Rank::Ace,
            11 => Rank::Jack,
            12 => Rank::Queen,
            13 => Rank::King,
            n => Rank::Number(n),
        }
    }
}

#[derive(Debug)]
pub struct Card {
    pub suit: Suit,
    pub rank: Rank,
}

pub fn winner_card(card: &Card) -> bool {
    card.rank == Rank::Ace 
}

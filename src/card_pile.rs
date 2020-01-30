use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

extern crate rand;

use crate::card::Card;

#[derive(Debug, Serialize, Deserialize)]
pub struct CardPile {
    pub draw_pile: Vec<Card>,
    pub discard_pile: Vec<Card>,
}

impl CardPile {
    pub fn new(cards: &mut Vec<Card>) -> CardPile {
        let mut rng = rand::thread_rng();
        &cards.shuffle(&mut rng);
        CardPile {
            draw_pile: cards.to_vec(),
            discard_pile: Vec::new(),
        }
    }

    pub fn draw_cards(&mut self, count: usize) -> Vec<Card> {
        vec![0; count].iter().map(|_| self.draw_card()).collect()
    }

    pub fn draw_card(&mut self) -> Card {
        // try drawing a card
        match self.draw_pile.pop() {
            Some(card) => card,
            None => {
                // shuffle the discard pile into the draw pile
                let mut rng = rand::thread_rng();
                &self.discard_pile.shuffle(&mut rng);
                self.draw_pile.append(&mut self.discard_pile);
                // finally draw a card 
                match self.draw_pile.pop() {
                    Some(card) => card,
                    // draw pile is empty, panic
                    None => panic!("Cannot draw card, discard pile and draw pile is empty!")
                }
            }
        }
    }

    pub fn discard(&mut self, card: Card) -> () {
        self.discard_pile.push(card)
    }

    pub fn discard_cards(&mut self, cards: &mut Vec<Card>) -> () {
        self.discard_pile.append(cards)
    }
}

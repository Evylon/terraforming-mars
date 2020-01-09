use rand::seq::SliceRandom;

extern crate rand;

pub use crate::card::Card;

pub struct CardPile {
    draw_pile: Vec<Card>,
    discard_pile: Vec<Card>,
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
}

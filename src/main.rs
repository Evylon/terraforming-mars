use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

extern crate rand;
use rand::seq::SliceRandom;

pub mod game_state;
pub mod card;

use card::Card;

fn main() {
    // load cards
    let mut card_pile = CardPile::new();
    let path = Path::new("cards/");
    for entry in path.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().unwrap() == "json" {
                let mut file = File::open(&path).unwrap();
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                card_pile.draw_pile.push(serde_json::from_str(&mut content).unwrap());
            }
        }
    }

    // init game
    let mut my_state = game_state::GameState::new();
    my_state.add_player();
    my_state.add_player();
    my_state.add_player();

    // assign start cards
    for player in my_state.players.iter_mut() {
        card_pile.draw_cards(player, 10);
    }
    println!("{:?}", my_state);
}

struct CardPile {
    draw_pile: Vec<Card>,
    discard_pile: Vec<Card>,
}

impl CardPile {
    fn new() -> CardPile {
        CardPile {
            draw_pile: Vec::new(),
            discard_pile: Vec::new(),
        }
    }

    fn draw_card(&mut self) -> Card {
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

    fn draw_cards(&mut self, player: &mut game_state::Player, count: u32) -> () {
        for _ in 0..count {
            player.hand.push(self.draw_card());
        };
    }
}

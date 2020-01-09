use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub mod game_state;
pub mod card;
pub mod card_pile;

use card_pile::CardPile;

fn main() {
    // load cards
    let mut all_cards = Vec::new();
    let path = Path::new("cards/");
    for entry in path.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().unwrap() == "json" {
                let mut file = File::open(&path).unwrap();
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                all_cards.push(serde_json::from_str(&mut content).unwrap());
                
            }
        }
    }
    let mut card_pile = CardPile::new(&mut all_cards);

    // init game
    let mut my_state = game_state::GameState::new();
    my_state.add_player();
    my_state.add_player();
    my_state.add_player();

    // assign start cards
    for player in my_state.players.iter_mut() {
        player.draw_cards(&mut card_pile, 10);
    }
    println!("{:?}", my_state);
}

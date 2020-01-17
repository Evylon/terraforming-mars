use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use undo::Record;

pub mod game_state;
pub mod card;
pub mod player;
pub mod card_pile;
pub mod commands;

fn main() {
    // load cards
    let mut all_cards = Vec::<>::new();
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
    // init game
    let used_decks = vec![card::Deck::Basic];
    let mut my_state = game_state::GameState::new(all_cards.as_mut(), used_decks.as_ref());
    for _ in 0..3 {
        my_state.add_player();
    }
    let mut record = Record::builder().build(my_state);
    record.apply(commands::AddResources{player_id: 0, rescs: vec![player::Resource::Energy; 2]}).unwrap();
    println!("{:?}", record.as_target().players);
}

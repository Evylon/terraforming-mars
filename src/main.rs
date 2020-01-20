use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

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
    for _ in 0..2 {
        my_state.add_player();
    }
    let mut state_machine = commands::StateMachine::new(my_state);
    state_machine.apply(commands::AddResources{player_id: 0, rescs: vec![player::Resource::MegaCredits; 30]}).unwrap();
    println!("{:?}", state_machine.get_state().players[0]);
    state_machine.advance_phase();
    println!("{:?}", state_machine.get_state().players[0]);
    let mut card_ids: Vec<String> = state_machine.get_state().players[0].research_queue.iter().map(|c| c.id.to_owned()).collect();
    let research_ids = card_ids.split_off(card_ids.len() / 2);
    state_machine.apply(commands::ResearchCards{player_id: 0, card_ids: research_ids}).unwrap();
    println!("{:?}", state_machine.get_state().players[0]);
    state_machine.apply(commands::DiscardResearch{player_id: 0, card_ids: card_ids}).unwrap();
    println!("{:?}", state_machine.get_state().players[0]);
}

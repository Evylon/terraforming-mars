use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

mod state_machine;
mod card;
mod commands;
mod game_state;
mod player;
mod card_pile;

use crate::state_machine::StateMachine;
use crate::game_state::{GameState, Phase};
use crate::card::Deck;
use crate::commands::{ChooseCorporation, ResearchCards, CmdWrapper};

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
    let used_decks = vec![Deck::Basic];
    let my_state = GameState::new(all_cards.as_mut(), used_decks.as_ref(), 2);
    let mut state_machine = StateMachine::new(my_state, all_cards);
    state_machine.advance_phase().unwrap();

    for player_id in 0..2 {
        let card = state_machine.get_state().get_player(player_id).hand.first().unwrap();
        let cmd = ChooseCorporation{player_id: player_id, card_id: card.id.to_owned()};
        state_machine.apply(CmdWrapper::ChooseCorporation(cmd)).unwrap();
        let mut card_ids: Vec<String> = state_machine.get_state().players[player_id].research_queue.iter().map(|c| c.id.to_owned()).collect();
        let research_ids = card_ids.split_off(card_ids.len() / 2);
        state_machine.apply(CmdWrapper::ResearchCards(ResearchCards{player_id: player_id, card_ids: research_ids})).unwrap();
        println!("{:?}", state_machine.get_state().players[player_id].corporation.as_ref().unwrap().production);
        println!("{:?}", state_machine.get_state().players[player_id].production);
    }
    // advance to action phase
    state_machine.advance_phase().unwrap();
    // test advancing to production phase
    state_machine.advance_phase().unwrap();
    // after production state should automatically transition into research phase
    assert_eq!(state_machine.get_state().phase, Phase::Research);
    println!("{:?}", state_machine.get_state().players[0]);
}

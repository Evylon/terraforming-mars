use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub mod game_state;
pub mod patent;
pub mod patent_pile;
pub mod player;

use patent_pile::PatentPile;

fn main() {
    // load patents
    let mut all_patents = Vec::new();
    let path = Path::new("patents/");
    for entry in path.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().unwrap() == "json" {
                let mut file = File::open(&path).unwrap();
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                all_patents.push(serde_json::from_str(&mut content).unwrap());
            }
        }
    }
    let mut patent_pile = PatentPile::new(&mut all_patents);

    // init game
    let mut my_state = game_state::GameState::new();
    my_state.add_player();
    my_state.add_player();
    my_state.add_player();

    // assign start patents
    for player in my_state.players.iter_mut() {
        player.draw_patents(&mut patent_pile, 10);
    }
    println!("{:?}", my_state);
}

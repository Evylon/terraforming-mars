use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub mod game_state;
pub mod project;
pub mod project_pile;
pub mod player;

use project_pile::ProjectPile;

fn main() {
    // load projects
    let mut all_projects = Vec::new();
    let path = Path::new("projects/");
    for entry in path.read_dir().unwrap() {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.extension().unwrap() == "json" {
                let mut file = File::open(&path).unwrap();
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                all_projects.push(serde_json::from_str(&mut content).unwrap());
            }
        }
    }
    let mut project_pile = ProjectPile::new(&mut all_projects);

    // init game
    let mut my_state = game_state::GameState::new();
    my_state.add_player();
    my_state.add_player();
    my_state.add_player();

    // assign start projects
    for player in my_state.players.iter_mut() {
        player.draw_projects(&mut project_pile, 10);
    }
    println!("{:?}", my_state);
}

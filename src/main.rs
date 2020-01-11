use std::path::Path;
use std::fs::File;
use std::io::prelude::*;

pub mod game_state;
pub mod project;
pub mod project_pile;
pub mod player;

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
    // init game
    let mut my_state = game_state::GameState::new(all_projects.as_mut());
    for _ in 0..3 {
        my_state.add_player();
    }
    my_state.advance_phase();
    println!("{:?}", my_state);
}

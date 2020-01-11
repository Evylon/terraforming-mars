use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize};

extern crate rand;

pub use crate::project::Project;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectPile {
    draw_pile: Vec<Project>,
    disproject_pile: Vec<Project>,
}

impl ProjectPile {
    pub fn new(projects: &mut Vec<Project>) -> ProjectPile {
        let mut rng = rand::thread_rng();
        &projects.shuffle(&mut rng);
        ProjectPile {
            draw_pile: projects.to_vec(),
            disproject_pile: Vec::new(),
        }
    }

    pub fn draw_project(&mut self) -> Project {
        // try drawing a project
        match self.draw_pile.pop() {
            Some(project) => project,
            None => {
                // shuffle the disproject pile into the draw pile
                let mut rng = rand::thread_rng();
                &self.disproject_pile.shuffle(&mut rng);
                self.draw_pile.append(&mut self.disproject_pile);
                // finally draw a project 
                match self.draw_pile.pop() {
                    Some(project) => project,
                    // draw pile is empty, panic
                    None => panic!("Cannot draw project, disproject pile and draw pile is empty!")
                }
            }
        }
    }
}

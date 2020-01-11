use rand::seq::SliceRandom;

extern crate rand;

pub use crate::patent::Patent;

pub struct PatentPile {
    draw_pile: Vec<Patent>,
    dispatent_pile: Vec<Patent>,
}

impl PatentPile {
    pub fn new(patents: &mut Vec<Patent>) -> PatentPile {
        let mut rng = rand::thread_rng();
        &patents.shuffle(&mut rng);
        PatentPile {
            draw_pile: patents.to_vec(),
            dispatent_pile: Vec::new(),
        }
    }

    pub fn draw_patent(&mut self) -> Patent {
        // try drawing a patent
        match self.draw_pile.pop() {
            Some(patent) => patent,
            None => {
                // shuffle the dispatent pile into the draw pile
                let mut rng = rand::thread_rng();
                &self.dispatent_pile.shuffle(&mut rng);
                self.draw_pile.append(&mut self.dispatent_pile);
                // finally draw a patent 
                match self.draw_pile.pop() {
                    Some(patent) => patent,
                    // draw pile is empty, panic
                    None => panic!("Cannot draw patent, dispatent pile and draw pile is empty!")
                }
            }
        }
    }
}

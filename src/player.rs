use serde::{Deserialize, Serialize};

pub use crate::patent::Patent;
pub use crate::patent_pile::PatentPile;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: u32,
    pub tr: i32,
    pub corporation: u32,
    pub inventory: Inventory,
    pub production: Production,
    pub hand: Vec<Patent>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Inventory {
    pub megacredits: u32,
    pub steel: u32,
    pub titanium: u32,
    pub plants: u32,
    pub energy: u32,
    pub heat: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Production {
    pub megacredits: i32,
    pub steel: i32,
    pub titanium: i32,
    pub plants: i32,
    pub energy: i32,
    pub heat: i32
}

impl Player {
    pub fn draw_patents(&mut self, patent_pile: &mut PatentPile, count: u32) -> () {
        for _ in 0..count {
            self.hand.push(patent_pile.draw_patent());
        };
    }
}

use serde::{Deserialize, Serialize};

pub use crate::card::Card;
pub use crate::card_pile::CardPile;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: u32,
    pub tr: i32,
    pub corporation: u32,
    pub inventory: Inventory,
    pub production: Production,
    pub hand: Vec<Card>,
    pub research_queue: Vec<Card>,
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
    // put projects into research queue, player has to pay 3 megacredits for each if they want to keep it
    pub fn research_projects(&mut self, project_pile: &mut CardPile, count: u32) -> () {
        for _ in 0..count {
            self.research_queue.push(project_pile.draw_card());
        }
    }

    // draw projects for free
    pub fn draw_projects(&mut self, project_pile: &mut CardPile, count: u32) -> () {
        for _ in 0..count {
            self.hand.push(project_pile.draw_card());
        };
    }

    pub fn new(id: u32) -> Player {
        Player {
            id: id,
            tr: 0,
            corporation: 0,
            inventory: Inventory {
                megacredits: 0,
                steel: 0,
                titanium: 0,
                plants: 0,
                energy: 0,
                heat: 0,
            },
            production: Production {
                megacredits: 0,
                steel: 0,
                titanium: 0,
                plants: 0,
                energy: 0,
                heat: 0,
            },
            hand: Vec::new(),
            research_queue: Vec::new(),
        }
    }
}

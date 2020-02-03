use serde::{Deserialize, Serialize};

use crate::card::Card;

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: usize,
    pub tf_rating: i32,
    pub action_state: ActionState,
    pub corporation: Option<Card>,
    pub inventory: Inventory,
    pub production: Production,
    pub hand: Vec<Card>,
    pub research_queue: Vec<Card>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ActionState {
    Acting(u8), Waiting, Passed,
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
    pub steel: u32,
    pub titanium: u32,
    pub plants: u32,
    pub energy: u32,
    pub heat: u32
}

impl Player {
    pub fn enqueue_research(&mut self, projects: &mut Vec<Card>) -> () {
        self.research_queue.append(projects);
    }

    pub fn draft_corporations(&mut self, corporations: &mut Vec<Card>) -> () {
        self.hand.append(corporations);
    }

    pub fn new(id: usize) -> Player {
        Player {
            id: id,
            tf_rating: 20,
            action_state: ActionState::Waiting,
            corporation: None,
            inventory: Inventory {
                megacredits: 0,
                steel: 0,
                titanium: 0,
                plants: 0,
                energy: 0,
                heat: 0,
            },
            production: Production {
                megacredits: 1,
                steel: 1,
                titanium: 1,
                plants: 1,
                energy: 1,
                heat: 1,
            },
            hand: Vec::new(),
            research_queue: Vec::new(),
        }
    }
}

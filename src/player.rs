use serde::{Deserialize, Serialize};
use std::fmt;

pub use crate::card::Card;
pub use crate::card_pile::CardPile;

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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Resource {
    MegaCredits(i32), Steel(i32), Titanium(i32), Plants(i32), Energy(i32), Heat(i32), Special,
}

impl fmt::Display for Resource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Resource {
    pub fn from_string(number: &String, resource: Resource) -> Option<Resource> {
        match number.parse::<i32>() {
            Err(_e) => match number.as_ref() {
                "Ref" => Some(Resource::Special),
                "No" => None,
                "Floaters" => None, // TODO this is an extension feature
                _ => panic!("Cannot convert CSVCard resource to {} Resources::{}!", number, resource),
            }
            Ok(count) => match resource {
                Resource::MegaCredits(_) => Some(Resource::MegaCredits(count)),
                Resource::Steel(_) => Some(Resource::Steel(count)),
                Resource::Titanium(_) => Some(Resource::Titanium(count)),
                Resource::Plants(_) => Some(Resource::Plants(count)),
                Resource::Energy(_) => Some(Resource::Energy(count)),
                Resource::Heat(_) => Some(Resource::Heat(count)),
                Resource::Special => None,
            }
        }
    }
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

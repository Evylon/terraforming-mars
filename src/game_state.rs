use serde::{Deserialize, Serialize};

pub use crate::player::*;
pub use crate::card::Card;
pub use crate::card_pile::CardPile;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub generation: u32,
    pub oxygen: u32,
    pub temperature: i32,
    pub oceans_placed: u32,
    pub tiles: Vec<Tile>,
    pub special_tiles: Vec<SpecialTile>,
    pub milestones: Vec<Milestone>,
    pub awards: Vec<Award>,
    pub cards_in_play: Vec<OwnedCard>,
    pub players: Vec<Player>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Tile {
    pub x: u32,
    pub z: u32,
    pub tile_type: TileType,
    pub name: String,
    pub resources: Vec<Resources>,
    pub reserved: TileType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TileType {
    Empty,
    City,
    Greenery,
    Ocean,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecialTile {
    pub tile_type: TileType,
    pub name: String,
    pub resources: Vec<Resources>,
    pub reserved: TileType,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Resources {
    Steel,
    Titanium,
    Card,
    Plant,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OwnedCard {
    pub card: Card,
    pub owner: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Award {
    pub name: Awards,
    pub owner: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Awards {
    Landlord,
    Banker,
    Scientist,
    Thermalist,
    Miner,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Milestone {
    pub name: Milestones,
    pub owner: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Milestones {
    Terraformer,
    Mayor,
    Gardener,
    Builder,
    Planner,
}

impl GameState {
    pub fn new() -> GameState {
        return GameState {
            generation: 0,
            oxygen: 0,
            temperature: -30,
            oceans_placed: 0,
            tiles: vec![
                Tile {x: 0, z: 4, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Steel, Resources::Steel], reserved: TileType::Empty},
                Tile {x: 0, z: 5, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Steel, Resources::Steel], reserved: TileType::Ocean},
                Tile {x: 0, z: 6, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 0, z: 7, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Card], reserved: TileType::Ocean},
                Tile {x: 0, z: 8, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Ocean},
                Tile {x: 1, z: 3, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 1, z: 4, tile_type: TileType::Empty, name: "Tharsis Tholus".to_owned(), resources: vec![Resources::Steel], reserved: TileType::Empty},
                Tile {x: 1, z: 5, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 1, z: 6, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 1, z: 7, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 1, z: 8, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Card, Resources::Card], reserved: TileType::Ocean},
                Tile {x: 2, z: 2, tile_type: TileType::Empty, name: "Ascraeus Mons".to_owned(), resources: vec![Resources::Card], reserved: TileType::Empty},
                Tile {x: 2, z: 3, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 2, z: 4, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 2, z: 5, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 2, z: 6, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 2, z: 7, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 2, z: 8, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Steel], reserved: TileType::Empty},
                Tile {x: 3, z: 1, tile_type: TileType::Empty, name: "Pavonis Mons".to_owned(), resources: vec![Resources::Plant, Resources::Titanium], reserved: TileType::Empty},
                Tile {x: 3, z: 2, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant], reserved: TileType::Empty},
                Tile {x: 3, z: 3, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant], reserved: TileType::Empty},
                Tile {x: 3, z: 4, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant], reserved: TileType::Empty},
                Tile {x: 3, z: 5, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant, Resources::Plant], reserved: TileType::Empty},
                Tile {x: 3, z: 6, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant], reserved: TileType::Empty},
                Tile {x: 3, z: 7, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant], reserved: TileType::Empty},
                Tile {x: 3, z: 8, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant, Resources::Plant], reserved: TileType::Ocean},
                Tile {x: 4, z: 0, tile_type: TileType::Empty, name: "Arsia Mons".to_owned(), resources: vec![Resources::Plant, Resources::Plant], reserved: TileType::Empty},
                Tile {x: 4, z: 1, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant, Resources::Plant], reserved: TileType::Empty},
                Tile {x: 4, z: 2, tile_type: TileType::Empty, name: "Noctis City".to_owned(), resources: vec![Resources::Plant, Resources::Plant], reserved: TileType::City},
                Tile {x: 4, z: 3, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant, Resources::Plant], reserved: TileType::Ocean},
                Tile {x: 4, z: 4, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant, Resources::Plant], reserved: TileType::Ocean},
                Tile {x: 4, z: 5, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant, Resources::Plant], reserved: TileType::Ocean},
                Tile {x: 4, z: 6, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant, Resources::Plant], reserved: TileType::Empty},
                Tile {x: 4, z: 7, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant, Resources::Plant], reserved: TileType::Empty},
                Tile {x: 4, z: 8, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant, Resources::Plant], reserved: TileType::Empty},
                Tile {x: 5, z: 0, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant], reserved: TileType::Empty},
                Tile {x: 5, z: 1, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant, Resources::Plant], reserved: TileType::Empty},
                Tile {x: 5, z: 2, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant], reserved: TileType::Empty},
                Tile {x: 5, z: 3, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant], reserved: TileType::Empty},
                Tile {x: 5, z: 4, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant], reserved: TileType::Empty},
                Tile {x: 5, z: 5, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant], reserved: TileType::Ocean},
                Tile {x: 5, z: 6, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant], reserved: TileType::Ocean},
                Tile {x: 5, z: 7, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Plant], reserved: TileType::Ocean},
                Tile {x: 6, z: 0, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 6, z: 1, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 6, z: 2, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 6, z: 3, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 6, z: 4, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 6, z: 5, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Steel], reserved: TileType::Empty},
                Tile {x: 6, z: 6, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 7, z: 0, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Steel, Resources::Steel], reserved: TileType::Empty},
                Tile {x: 7, z: 1, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 7, z: 2, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Card], reserved: TileType::Empty},
                Tile {x: 7, z: 3, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Card], reserved: TileType::Empty},
                Tile {x: 7, z: 4, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 7, z: 5, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Titanium], reserved: TileType::Empty},
                Tile {x: 8, z: 0, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Steel], reserved: TileType::Empty},
                Tile {x: 8, z: 1, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Steel, Resources::Steel], reserved: TileType::Empty},
                Tile {x: 8, z: 2, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 8, z: 3, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![], reserved: TileType::Empty},
                Tile {x: 8, z: 4, tile_type: TileType::Empty, name: "".to_owned(), resources: vec![Resources::Titanium, Resources::Titanium], reserved: TileType::Ocean}
            ],
            special_tiles: vec![
                SpecialTile {name: "Phobos Space Haven".to_owned(), tile_type: TileType::Empty, resources: vec![], reserved: TileType::City},
                SpecialTile {name: "Ganymede Colony".to_owned(), tile_type: TileType::Empty, resources: vec![], reserved: TileType::City}
            ],
            milestones: vec![
                Milestone {name: Milestones::Terraformer, owner: -1},
                Milestone {name: Milestones::Mayor, owner: -1},
                Milestone {name: Milestones::Gardener, owner: -1},
                Milestone {name: Milestones::Builder, owner: -1},
                Milestone {name: Milestones::Planner, owner: -1},
            ],
            awards: vec![
                Award {name: Awards::Landlord, owner: -1},
                Award {name: Awards::Banker, owner: -1},
                Award {name: Awards::Scientist, owner: -1},
                Award {name: Awards::Thermalist, owner: -1},
                Award {name: Awards::Miner, owner: -1},
            ],
            cards_in_play: vec![],
            players: vec![]
        };
    }

    pub fn add_player(&mut self) -> () {
        self.players.push(
            Player {
                id: self.players.len() as u32,
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
            }
        )
    }
}

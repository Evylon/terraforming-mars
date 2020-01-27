use serde::{Deserialize, Serialize};
use rand::prelude::*;

pub use crate::player::{Player};
pub use crate::card::{Card, Deck, CardType};
pub use crate::card_pile::CardPile;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameState {
    pub phase: Phase,
    pub generation: u32,
    pub start_player: usize,
    pub active_player: usize,
    pub oxygen: u32,
    pub temperature: i32,
    pub oceans_placed: u32,
    pub tiles: Vec<Tile>,
    pub special_tiles: Vec<SpecialTile>,
    pub milestones: Vec<Milestone>,
    pub awards: Vec<Award>,
    pub cards_in_play: Vec<OwnedCard>,
    pub players: Vec<Player>,
    pub project_pile: CardPile,
    pub corporation_pile: CardPile,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Phase {
    Init, Setup, Research, Action, Production,
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
    pub owner: usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Award {
    pub name: Awards,
    pub owner: Option<usize>,
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
    pub owner: Option<usize>,
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
    pub fn add_player(&mut self) -> () {
        self.players.push(
            Player::new(self.players.len())
        )
    }

    pub fn get_player(&self, id: usize) -> &Player {
        &self.players[id]
    }

    pub fn get_player_mut(&mut self, id: usize) -> &mut Player {
        &mut self.players[id]
    }

    pub fn new(cards: &mut Vec<Card>, used_decks: &Vec<Deck>, player_count: usize) -> GameState {
        let deck: Vec<Card> = cards.iter().filter(|card| used_decks.contains(&card.deck)).cloned().collect();
        let mut projects: Vec<Card> = deck.iter().filter(|card| card.card_type != CardType::Corporation).cloned().collect();
        let mut corporations: Vec<Card> = deck.iter().filter(|card| card.card_type == CardType::Corporation).cloned().collect();
        let start_player_id = rand::thread_rng().next_u32() as usize % player_count;
        let mut state = GameState {
            phase: Phase::Init,
            generation: 0,
            start_player: start_player_id,
            active_player: start_player_id,
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
                Milestone {name: Milestones::Terraformer, owner: None},
                Milestone {name: Milestones::Mayor, owner: None},
                Milestone {name: Milestones::Gardener, owner: None},
                Milestone {name: Milestones::Builder, owner: None},
                Milestone {name: Milestones::Planner, owner: None},
            ],
            awards: vec![
                Award {name: Awards::Landlord, owner: None},
                Award {name: Awards::Banker, owner: None},
                Award {name: Awards::Scientist, owner: None},
                Award {name: Awards::Thermalist, owner: None},
                Award {name: Awards::Miner, owner: None},
            ],
            cards_in_play: vec![],
            players: vec![],
            project_pile: CardPile::new(projects.as_mut()),
            corporation_pile: CardPile::new(corporations.as_mut()),
        };
        for _ in 0..player_count {
            state.add_player();
        }
        return state;
    }
}

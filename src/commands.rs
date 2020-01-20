use undo::{Command, Record};
use std::{error::Error, fmt};

use crate::player::{Player, Resource};
use crate::game_state::*;
use crate::card::Card;

pub struct DrawCards{pub player_id: usize, pub count: usize, pub is_project: bool}

impl Command<GameState> for DrawCards {
    // pile: [c1, c2, c3] -> hand: [c3, c2, c1]
    // draw_cards 'reverses' the card order. the last card in the vector is the last one drawn
    fn apply(&mut self, game_state: &mut GameState) -> undo::Result {
        if self.is_project {
            let mut cards = game_state.project_pile.draw_cards(self.count);
            let player = game_state.get_player(self.player_id);
            player.enqueue_research(cards.as_mut());
        } else {
            let mut cards = game_state.corporation_pile.draw_cards(self.count);
            let player = game_state.get_player(self.player_id);
            player.draft_corporations(cards.as_mut());
        }
        Ok(())
    }
    // hand: [c3, c2, c1] -> pile: [c1, c2, c3]
    fn undo(&mut self, game_state: &mut GameState) -> undo::Result {
        let player = game_state.get_player(self.player_id);
        let draw_range = player.hand.len()-self.count..;
        if self.is_project {
            let mut cards = player.research_queue.drain(draw_range).rev().collect::<Vec<Card>>();
            game_state.project_pile.draw_pile.append(cards.as_mut());
        } else {
            let mut cards = player.hand.drain(draw_range).rev().collect::<Vec<Card>>();
            game_state.corporation_pile.draw_pile.append(cards.as_mut());
        }
        Ok(())
    }
}

pub struct ResearchCards{pub player_id: usize, pub card_ids: Vec<String>}

const CARD_COST: u32 = 3;

impl Command<GameState> for ResearchCards {
    fn apply(&mut self, game_state: &mut GameState) -> undo::Result {
        let player = game_state.get_player(self.player_id);
        // validate consistent ids in research_queue and card_ids
        if player.research_queue.iter().filter(|c| self.card_ids.contains(&c.id)).count() != self.card_ids.len() {
            return Err(Box::new(CannotExecute{reason: "card_ids and research_queue did not match".to_owned()}));
        }
        // check if player has sufficient funds
        if player.inventory.megacredits < self.card_ids.len() as u32 * CARD_COST {
            return Err(Box::new(CannotExecute{reason: "Cannot buy cards, not enough Megacredits!".to_owned()}));
        }
        // move cards from research_queue to player.hand while retaining the projects not researched
        let (mut research_queue, mut not_researched): (Vec<Card>, Vec<Card>) = player.research_queue.drain(..).partition(|c| self.card_ids.contains(&c.id));
        player.research_queue.append(not_researched.as_mut());
        player.hand.append(research_queue.as_mut());
        Ok(())
    }

    fn undo(&mut self, game_state: &mut GameState) -> undo::Result {
        let player = game_state.get_player(self.player_id);
        // new cards are always appended. just return last n cards to the research_queue
        let first_idx = player.hand.len() - self.card_ids.len();
        let mut cards = player.hand.drain(first_idx..).collect::<Vec<Card>>();
        player.research_queue.append(cards.as_mut());
        Ok(())
    }
}

pub struct DiscardResearch{pub player_id: usize, pub card_ids: Vec<String>}

impl Command<GameState> for DiscardResearch {
    fn apply(&mut self, game_state: &mut GameState) -> undo::Result {
        let player = game_state.get_player(self.player_id);
        // validate consistent ids in research_queue and card_ids
        if player.research_queue.iter().filter(|c| self.card_ids.contains(&c.id)).count() != self.card_ids.len() {
            return Err(Box::new(CannotExecute{reason: "card_ids and research_queue did not match".to_owned()}));
        }
        // collect cards to discard while retaining the cards not discarded
        let (mut discard_queue, mut not_discarded): (Vec<Card>, Vec<Card>) = player.research_queue.drain(..).partition(|c| self.card_ids.contains(&c.id));
        player.research_queue.append(not_discarded.as_mut());
        game_state.project_pile.discard_cards(discard_queue.as_mut());
        Ok(())
    }

    fn undo(&mut self, game_state: &mut GameState) -> undo::Result {
        // new cards are always appended. just return last n cards to the research_queue
        let first_idx = game_state.project_pile.discard_pile.len() - self.card_ids.len();
        let mut cards = game_state.project_pile.discard_pile.drain(first_idx..).collect::<Vec<Card>>();
        game_state.get_player(self.player_id).research_queue.append(cards.as_mut());
        Ok(())
    }
}

pub struct AddResources{pub player_id: usize, pub rescs: Vec<Resource>}

fn increment_resource(player: &mut Player, res: &Resource) -> () {
    match res {
        Resource::MegaCredits => player.inventory.megacredits += 1,
        Resource::Steel => player.inventory.steel += 1,
        Resource::Titanium => player.inventory.titanium += 1,
        Resource::Plants => player.inventory.plants += 1,
        Resource::Energy => player.inventory.energy += 1,
        Resource::Heat => player.inventory.heat += 1,
    }
}

fn decrement_resource(player: &mut Player, res: &Resource) -> () {
    match res {
        Resource::MegaCredits => player.inventory.megacredits -= 1,
        Resource::Steel => player.inventory.steel -= 1,
        Resource::Titanium => player.inventory.titanium -= 1,
        Resource::Plants => player.inventory.plants -= 1,
        Resource::Energy => player.inventory.energy -= 1,
        Resource::Heat => player.inventory.heat -= 1,
    }
}

impl Command<GameState> for AddResources {
    fn apply(&mut self, game_state: &mut GameState) -> undo::Result {
        let player = game_state.get_player(self.player_id);
        self.rescs.iter().for_each(|res| increment_resource(player, res));
        Ok(())
    }

    fn undo(&mut self, game_state: &mut GameState) -> undo::Result {
        let player = game_state.get_player(self.player_id);
        self.rescs.iter().for_each(|res| decrement_resource(player, res));
        Ok(())
    }
}

pub struct StateMachine {
    record: Record<GameState>,
}

impl StateMachine {
    pub fn new(state: GameState) -> StateMachine {
        StateMachine {
            record: Record::builder().build(state),
        }
    }

    pub fn get_state(&self) -> &GameState {
        self.record.as_target()
    }

    pub fn apply(&mut self, command: impl Command<GameState> + 'static) -> undo::Result {
        self.record.apply(command)
    }

    pub fn advance_phase(&mut self) -> () {
        // TODO implement action phase
        match self.get_state().phase {
            Phase::Init => self.setup_phase(),
            Phase::Setup => self.transition_setup_to_action().unwrap(),
            Phase::Research => self.research_phase(),
            Phase::Action => (), //TODO
            Phase::Production => self.production_phase(),
        }
    }

    fn setup_phase(&mut self) -> () {
        let player_ids = self.get_state().players.iter().map(|p| p.id).collect::<Vec<usize>>();
        for id in player_ids {
            // assign corporations
            self.record.apply(DrawCards{player_id: id, count: 2, is_project: false}).unwrap();
            // assign start cards
            self.record.apply(DrawCards{player_id: id, count: 10, is_project: true}).unwrap();
        }
    }

    fn transition_setup_to_action(&mut self) -> Result<(), ()> {
        // all players have to choose a corporation
        if self.get_state().players.iter().any(|p| p.corporation.is_none()) {
            return Err(());
        }
        // players may hold only projects, no corporations
        if self.get_state().players.iter().flat_map(|p| &p.hand).any(|card| card.card_type == CardType::Corporation) {
            return Err(());
        }
        // all players have to empty their research queue
        if !self.get_state().players.iter().all(|p| p.research_queue.is_empty()) {
            return Err(());
        }
        Ok(())
    }

    fn research_phase(&mut self) -> () {
        let player_ids = self.get_state().players.iter().map(|p| p.id).collect::<Vec<usize>>();
        for id in player_ids {
            self.record.apply(DrawCards{player_id: id, count: 10, is_project: true}).unwrap();
        }
    }

    fn production_phase(&mut self) -> () {
        // TODO
    }
}

#[derive(Debug)]
pub struct CannotExecute{reason: String}

impl fmt::Display for CannotExecute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}
impl Error for CannotExecute {}

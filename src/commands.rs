use undo::{Command, Record, Chain};
use std::{error::Error, fmt};

use crate::player::{Resource, ActionState};
use crate::game_state::{GameState, OwnedCard, Phase};
use crate::card::{Card, CardType};

const CHAIN_RESEARCH_ID: u32 = 1;
const CHAIN_PLAY_CARD_ID: u32 = 2;

pub struct DrawCards{pub player_id: usize, pub count: usize, pub card_type: CardType}

impl Command<GameState> for DrawCards {
    // pile: [c1, c2, c3] -> hand: [c3, c2, c1]
    // draw_cards 'reverses' the card order. the last card in the vector is the last one drawn
    fn apply(&mut self, game_state: &mut GameState) -> undo::Result {
        match self.card_type {
            CardType::Corporation => {
                let mut cards = game_state.corporation_pile.draw_cards(self.count);
                let player = game_state.get_player_mut(self.player_id);
                player.draft_corporations(cards.as_mut());
            }
            _ => {
                let mut cards = game_state.project_pile.draw_cards(self.count);
                let player = game_state.get_player_mut(self.player_id);
                player.enqueue_research(cards.as_mut());
            }
        }
        Ok(())
    }
    // hand: [c3, c2, c1] -> pile: [c1, c2, c3]
    fn undo(&mut self, game_state: &mut GameState) -> undo::Result {
        let player = game_state.get_player_mut(self.player_id);
        let draw_range = player.hand.len()-self.count..;
        match self.card_type {
            CardType::Corporation => {
                let mut cards = player.hand.drain(draw_range).rev().collect::<Vec<Card>>();
                game_state.corporation_pile.draw_pile.append(cards.as_mut());
            }
            _ => {
                let mut cards = player.research_queue.drain(draw_range).rev().collect::<Vec<Card>>();
                game_state.project_pile.draw_pile.append(cards.as_mut());
            }
        }
        Ok(())
    }
}

pub struct PlayCard{pub owner_id: usize, pub card_id: String, pub target_id: Option<usize>}

impl Command<GameState> for PlayCard {
    fn apply(&mut self, game_state: &mut GameState) -> undo::Result {
        let player = game_state.get_player_mut(self.owner_id);
        let card = match player.hand.iter().position(|c| c.id == self.card_id) {
            Some(idx) => player.hand.remove(idx),
            None => return Err(Box::new(CannotExecute{reason: format!("Card {} not found in player {}'s hand!", self.card_id, self.owner_id)})),
        };
        match player.inventory.megacredits.checked_sub(card.cost) {
            Some(result) => player.inventory.megacredits = result,
            None => return Err(Box::new(CannotExecute{
                reason: format!("Insufficient funds! Player {} need {} Megacredits to play card {}!", self.owner_id, card.cost, card.id)
            })),
        };
        // TODO
        // check requirements
        game_state.cards_in_play.push(OwnedCard{card: card, owner: self.owner_id});
        Ok(())
    }

    fn undo(&mut self, game_state: &mut GameState) -> undo::Result {
        let owned_card = match game_state.cards_in_play.iter().position(|c| c.card.id == self.card_id) {
            Some(idx) => game_state.cards_in_play.remove(idx),
            None => return Err(Box::new(CannotExecute{reason: format!("Card {} not found in cards_in_player!", self.card_id)})),
        };
        let player = game_state.get_player_mut(self.owner_id);
        player.inventory.megacredits += owned_card.card.cost;
        player.hand.push(owned_card.card);
        Ok(())
    }

    fn merge(&self) -> undo::Merge {
        undo::Merge::If(CHAIN_PLAY_CARD_ID)
    }
}

pub struct ChooseCorporation{pub player_id: usize, pub card_id: String}

impl Command<GameState> for ChooseCorporation {
    fn apply(&mut self, game_state: &mut GameState) -> undo::Result {
        if game_state.phase != Phase::Setup {
            return Err(Box::new(CannotExecute{reason: "Can only select corporation in setup phase!".to_owned()}));
        }
        let player = game_state.get_player_mut(self.player_id);
        let (mut chosen, mut rejected): (Vec<Card>, Vec<Card>) = player.hand.drain(..).partition(|c| c.id == self.card_id);
        match chosen.pop() {
            Some(card) => player.corporation = Some(card),
            None => return Err(Box::new(CannotExecute{reason: format!("Corporation {} not found in player {}'s hand!", self.card_id, self.player_id)})),
        }
        game_state.corporation_pile.discard_cards(rejected.as_mut());
        Ok(())
    }

    fn undo(&mut self, game_state: &mut GameState) -> undo::Result {
        // players always choose from 2 corporations
        let mut coorps = vec![game_state.corporation_pile.discard_pile.pop().unwrap()];
        let player = game_state.get_player_mut(self.player_id);
        coorps.push(player.corporation.take().unwrap());
        player.hand.append(coorps.as_mut());
        Ok(())
    }

    fn merge(&self) -> undo::Merge {
        undo::Merge::If(CHAIN_PLAY_CARD_ID)
    }
}

pub struct ResearchCards{pub player_id: usize, pub card_ids: Vec<String>}

const CARD_COST: u32 = 3;

impl Command<GameState> for ResearchCards {
    fn apply(&mut self, game_state: &mut GameState) -> undo::Result {
        let player = game_state.get_player_mut(self.player_id);
        // validate consistent ids in research_queue and card_ids
        if player.research_queue.iter().filter(|c| self.card_ids.contains(&c.id)).count() != self.card_ids.len() {
            return Err(Box::new(CannotExecute{reason: "card_ids and research_queue did not match".to_owned()}));
        }
        // check if player has sufficient funds
        match player.inventory.megacredits.checked_sub(self.card_ids.len() as u32 * CARD_COST) {
            Some(result) => player.inventory.megacredits = result,
            None => return Err(Box::new(CannotExecute{reason: "Cannot buy cards, not enough Megacredits!".to_owned()})),
        };
        // move cards from research_queue to player.hand while retaining the projects not researched
        let (mut research_queue, mut not_researched): (Vec<Card>, Vec<Card>) = player.research_queue.drain(..).partition(|c| self.card_ids.contains(&c.id));
        player.research_queue.append(not_researched.as_mut());
        player.hand.append(research_queue.as_mut());
        Ok(())
    }

    fn undo(&mut self, game_state: &mut GameState) -> undo::Result {
        let player = game_state.get_player_mut(self.player_id);
        // new cards are always appended. just return last n cards to the research_queue
        let first_idx = player.hand.len() - self.card_ids.len();
        let mut cards = player.hand.drain(first_idx..).collect::<Vec<Card>>();
        player.research_queue.append(cards.as_mut());
        Ok(())
    }

    fn merge(&self) -> undo::Merge {
        undo::Merge::If(CHAIN_RESEARCH_ID)
    }
}

struct DiscardResearch{pub player_id: usize, pub card_ids: Vec<String>}

impl Command<GameState> for DiscardResearch {
    fn apply(&mut self, game_state: &mut GameState) -> undo::Result {
        let player = game_state.get_player_mut(self.player_id);
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
        game_state.get_player_mut(self.player_id).research_queue.append(cards.as_mut());
        Ok(())
    }

    fn merge(&self) -> undo::Merge {
        undo::Merge::If(CHAIN_RESEARCH_ID)
    }
}

pub struct ModResources{pub player_id: usize, pub rescs: Vec<Resource>}

fn mod_inventory(inv: &mut u32, count: i32, player_id: usize, res_type: String) -> undo::Result {
    if count < 0 && count.abs() as u32 > *inv {
        Err(Box::new(CannotExecute{reason: format!("Insufficient {}! player {} needs {} but has only {}", res_type, player_id, -count, *inv)}))
    } else {
        *inv = (*inv as i32 + count) as u32;
        Ok(())
    }
}

impl Command<GameState> for ModResources {
    fn apply(&mut self, game_state: &mut GameState) -> undo::Result {
        let player = game_state.get_player_mut(self.player_id);
        for res in self.rescs.iter() {
            let result = match *res {
                Resource::MegaCredits(count) => mod_inventory(&mut player.inventory.megacredits, count, player.id, "Megacredits".to_owned()),
                Resource::Steel(count) => mod_inventory(&mut player.inventory.steel, count, player.id, "Steel".to_owned()),
                Resource::Titanium(count) => mod_inventory(&mut player.inventory.titanium, count, player.id, "Titanium".to_owned()),
                Resource::Plants(count) => mod_inventory(&mut player.inventory.plants, count, player.id, "Plants".to_owned()),
                Resource::Energy(count) => mod_inventory(&mut player.inventory.energy, count, player.id, "Energy".to_owned()),
                Resource::Heat(count) => mod_inventory(&mut player.inventory.heat, count, player.id, "Heat".to_owned()),
                Resource::Special => continue, // TODO
            };
            match result {
                Err(err) => return Err(err),
                Ok(()) => ()
            };
        }
        Ok(())
    }

    fn undo(&mut self, game_state: &mut GameState) -> undo::Result {
        let player = game_state.get_player_mut(self.player_id);
        for res in self.rescs.iter() {
            // we can always undo
            match *res {
                Resource::MegaCredits(count) => player.inventory.megacredits = (player.inventory.megacredits as i32 - count) as u32,
                Resource::Steel(count) => player.inventory.steel = (player.inventory.steel as i32 - count) as u32,
                Resource::Titanium(count) => player.inventory.titanium = (player.inventory.titanium as i32 - count) as u32,
                Resource::Plants(count) => player.inventory.plants = (player.inventory.plants as i32 - count) as u32,
                Resource::Energy(count) => player.inventory.energy = (player.inventory.energy as i32 - count) as u32,
                Resource::Heat(count) => player.inventory.heat = (player.inventory.heat as i32 - count) as u32,
                Resource::Special => continue, // TODO 
            };
        }
        Ok(())
    }

    fn merge(&self) -> undo::Merge {
        undo::Merge::If(CHAIN_PLAY_CARD_ID)
    }
}

pub struct ModProduction{pub player_id: usize, pub rescs: Vec<Resource>}

fn mod_production(prod: &mut u32, count: i32, player_id: usize, res_type: String) -> undo::Result {
    if count < 0 && count.abs() as u32 > *prod {
        Err(Box::new(CannotExecute{reason: format!("Insufficient {} production! player {} needs {} but has only {}", res_type, player_id, -count, *prod)}))
    } else {
        *prod = (*prod as i32 + count) as u32;
        Ok(())
    }
}

impl Command<GameState> for ModProduction {
    fn apply(&mut self, game_state: &mut GameState) -> undo::Result {
        let player = game_state.get_player_mut(self.player_id);
        for res in self.rescs.iter() {
            // if res == Resource
            let result = match *res {
                // special case MegeCredits can be down to -5 production
                Resource::MegaCredits(count) => {
                    if count + player.production.megacredits < -5 {
                        return Err(Box::new(CannotExecute{reason: format!("Insufficient Megacredits production! player {} needs {}", player.id, -5-count)}))
                    } else {
                        player.production.megacredits += count;
                        Ok(())
                    }
                },
                Resource::Steel(count) => mod_production(&mut player.production.steel, count, player.id, "Steel".to_owned()),
                Resource::Titanium(count) => mod_production(&mut player.production.titanium, count, player.id, "Titanium".to_owned()),
                Resource::Plants(count) => mod_production(&mut player.production.plants, count, player.id, "Plants".to_owned()),
                Resource::Energy(count) => mod_production(&mut player.production.energy, count, player.id, "Energy".to_owned()),
                Resource::Heat(count) => mod_production(&mut player.production.heat, count, player.id, "Heat".to_owned()),
                Resource::Special => continue, // TODO
            };
            match result {
                Err(err) => return Err(err),
                Ok(()) => (),
            };
        }
        Ok(())
    }

    fn undo(&mut self, game_state: &mut GameState) -> undo::Result {
        let player = game_state.get_player_mut(self.player_id);
        for res in self.rescs.iter() {
            // we can always undo
            match *res {
                // special case MegaCredits production can also be negative
                Resource::MegaCredits(count) => player.production.megacredits -= count,
                Resource::Steel(count) => player.production.steel = (player.production.steel as i32 - count) as u32,
                Resource::Titanium(count) => player.production.titanium = (player.production.titanium as i32 - count) as u32,
                Resource::Plants(count) => player.production.plants = (player.production.plants as i32 - count) as u32,
                Resource::Energy(count) => player.production.energy = (player.production.energy as i32 - count) as u32,
                Resource::Heat(count) => player.production.heat = (player.production.heat as i32 - count) as u32,
                Resource::Special => continue, // TODO 
            };
        }
        Ok(())
    }

    fn merge(&self) -> undo::Merge {
        undo::Merge::If(CHAIN_PLAY_CARD_ID)
    }
}

pub struct StateMachine {
    record: Record<GameState>,
    cards: Vec<Card>,
}

pub enum CmdWrapper {
    ModResources(ModResources),
    DrawCards(DrawCards),
    PlayCard(PlayCard),
    ResearchCards(ResearchCards),
    ChooseCorporation(ChooseCorporation),
}

impl StateMachine {
    pub fn new(state: GameState, cards: Vec<Card>) -> StateMachine {
        StateMachine {
            record: Record::builder().build(state),
            cards: cards,
        }
    }

    fn lookup_card<'a>(&'a self, card_id: String) -> &'a Card {
        match self.cards.iter().find(|c| c.id == card_id) {
            Some(card) => card,
            None => panic!(format!("Cannot find card {}", card_id)),
        }
    }

    pub fn get_state(&self) -> &GameState {
        self.record.as_target()
    }

    pub fn apply(&mut self, command: CmdWrapper) -> undo::Result {
        match command {
            CmdWrapper::ModResources(cmd) => self.record.apply(cmd),
            CmdWrapper::DrawCards(cmd) => self.record.apply(cmd),
            CmdWrapper::PlayCard(cmd) => self.play_card(cmd.owner_id, cmd.card_id.to_owned(), cmd),
            CmdWrapper::ResearchCards(cmd) => self.research_card(cmd),
            CmdWrapper::ChooseCorporation(cmd) => self.play_card(cmd.player_id, cmd.card_id.to_owned(), cmd),
        }
    }

    fn research_card(&mut self, command: ResearchCards) -> undo::Result {
        let research_queue = &self.get_state().get_player(command.player_id).research_queue;
        let discard_ids = research_queue.iter().map(|c| c.id.to_owned()).filter(|id| !command.card_ids.contains(&id)).collect();
        let discard_cmd = DiscardResearch{player_id: command.player_id, card_ids: discard_ids};
        let chain = Chain::new().join(command).join(discard_cmd);
        self.record.apply(chain)
    }

    fn play_card(&mut self, player_id: usize, card_id: String, command: impl Command<GameState> + 'static) -> undo::Result {
        let card = self.lookup_card(card_id);
        let rescs_cmd = ModResources{player_id: player_id, rescs: card.resources.to_owned()};
        let prod_cmd = ModProduction{player_id: player_id, rescs: card.production.to_owned()};
        let chain = Chain::new().join(command).join(rescs_cmd).join(prod_cmd);
        // TODO one-time Actions/effects
        self.record.apply(chain)
    }

    pub fn advance_phase(&mut self) -> undo::Result {
        // TODO implement action phase
        match self.get_state().phase {
            Phase::Init => self.setup_phase(),
            Phase::Setup => self.transition_to_action(),
            Phase::Research => self.transition_to_action(),
            Phase::Action => self.production_phase(),
            Phase::Production => self.research_phase(),
        }
    }

    fn setup_phase(&mut self) -> undo::Result {
        let player_ids = self.get_state().players.iter().map(|p| p.id).collect::<Vec<usize>>();
        let mut chain = Chain::new();
        for id in player_ids {
            // assign corporations
            chain = chain.join(DrawCards{player_id: id, count: 2, card_type: CardType::Corporation});
            // assign start cards
            chain = chain.join(DrawCards{player_id: id, count: 10, card_type: CardType::Project});
        }
        match self.record.apply(chain) {
            Ok(()) => self.record.as_mut_target().phase = Phase::Setup,
            Err(err) => return Err(err),
        };
        Ok(())
    }

    fn transition_to_action(&mut self) -> undo::Result {
        // all players have to choose a corporation
        if self.get_state().players.iter().any(|p| p.corporation.is_none()) {
            return Err(Box::new(CannotExecute{reason: "Cannot advance to Action phase, a player has not selected a corporation!".to_owned()}));
        }
        // players may hold only projects, no corporations
        if self.get_state().players.iter().flat_map(|p| &p.hand).any(|card| card.card_type == CardType::Corporation) {
            return Err(Box::new(CannotExecute{reason: "Cannot advance to Action phase, a player has a corporation card in hand!".to_owned()}));
        }
        // all players have to empty their research queue
        if !self.get_state().players.iter().all(|p| p.research_queue.is_empty()) {
            return Err(Box::new(CannotExecute{reason: "Cannot advance to Action phase, a player still has research enqueued!".to_owned()}));
        }
        self.record.as_mut_target().phase = Phase::Action;
        let start_player_id = self.get_state().start_player;
        self.record.as_mut_target().players[start_player_id].action_state = ActionState::Acting(2);
        self.record.as_mut_target().active_player = start_player_id;
        Ok(())
    }

    fn research_phase(&mut self) -> undo::Result {
        let player_ids = self.get_state().players.iter().map(|p| p.id).collect::<Vec<usize>>();
        let mut chain = Chain::new();
        for id in player_ids {
            chain = chain.join(DrawCards{player_id: id, count: 4, card_type: CardType::Project});
        }
        match self.record.apply(chain) {
            Ok(()) => self.record.as_mut_target().phase = Phase::Research,
            Err(err) => return Err(err),
        };
        Ok(())
    }

    fn production_phase(&mut self) -> undo::Result {
        let mut chain = Chain::new();
        for player in self.get_state().players.iter() {
            let rescs = vec![
                Resource::MegaCredits(player.production.megacredits + player.tf_rating),
                Resource::Steel(player.production.steel as i32),
                Resource::Titanium(player.production.titanium as i32),
                Resource::Plants(player.production.plants as i32),
                Resource::Energy(player.production.energy as i32 - player.inventory.energy as i32),
                Resource::Heat(player.production.heat as i32 + player.inventory.energy as i32)
            ];
            chain = chain.join(ModResources{player_id: player.id, rescs: rescs});
        }
        match self.record.apply(chain) {
            Ok(()) => {
                // TODO
                // reset marker on action cards
                self.record.as_mut_target().generation += 1;
                // wrapping increment start_player
                let old_start_player = self.get_state().start_player;
                self.record.as_mut_target().start_player = (old_start_player + 1) % self.get_state().players.len();
                // transition to research phase
                self.research_phase()
            }
            Err(err) => Err(err),
        }
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

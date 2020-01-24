use undo::{Command, Record};
use std::{error::Error, fmt};

use crate::player::Resource;
use crate::game_state::{GameState, OwnedCard, Phase};
use crate::card::{Card, CardType};

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
}

pub struct ModResources{pub player_id: usize, pub rescs: Vec<Resource>}

impl Command<GameState> for ModResources {
    fn apply(&mut self, game_state: &mut GameState) -> undo::Result {
        let player = game_state.get_player_mut(self.player_id);
        for res in self.rescs.iter() {
            let (inv, count, res_type) = match *res {
                Resource::MegaCredits(count) => (&mut player.inventory.megacredits, count, "Megacredits"),
                Resource::Steel(count) => (&mut player.inventory.steel, count, "Steel"),
                Resource::Titanium(count) => (&mut player.inventory.titanium, count, "Titanium"),
                Resource::Plants(count) => (&mut player.inventory.plants, count, "Plants"),
                Resource::Energy(count) => (&mut player.inventory.energy, count, "Energy"),
                Resource::Heat(count) => (&mut player.inventory.heat, count, "Heat"),
                Resource::Special => continue, // TODO
            };
            if count < 0 && count.abs() as u32 > *inv {
                return Err(Box::new(CannotExecute{reason: format!("Insufficient {}! player {} needs {} but has only {}", res_type, player.id, -count, *inv)}));
            }
            *inv = (*inv as i32 + count) as u32;
        }
        Ok(())
    }

    fn undo(&mut self, game_state: &mut GameState) -> undo::Result {
        let player = game_state.get_player_mut(self.player_id);
        for res in self.rescs.iter() {
            // we can always undo
            let (inv, count) = match *res {
                Resource::MegaCredits(count) => (&mut player.inventory.megacredits, count),
                Resource::Steel(count) => (&mut player.inventory.steel, count),
                Resource::Titanium(count) => (&mut player.inventory.titanium, count),
                Resource::Plants(count) => (&mut player.inventory.plants, count),
                Resource::Energy(count) => (&mut player.inventory.energy, count),
                Resource::Heat(count) => (&mut player.inventory.heat, count),
                Resource::Special => continue, // TODO 
            };
            *inv = (*inv as i32 - count) as u32;
        }
        Ok(())
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
        let mut queue = self.record.queue();
        queue.apply(command);
        queue.apply(discard_cmd);
        queue.commit()
    }

    fn play_card(&mut self, player_id: usize, card_id: String, command: impl Command<GameState> + 'static) -> undo::Result {
        let card = self.lookup_card(card_id);
        let rescs_cmd = ModResources{player_id: player_id, rescs: card.resources.to_owned()};
        let mut queue = self.record.queue();
        queue.apply(command);
        queue.apply(rescs_cmd);
        // TODO production
        // TODO one-time Actions/effects
        queue.commit()
    }

    pub fn advance_phase(&mut self) -> () {
        // TODO implement action phase
        match self.get_state().phase {
            Phase::Init => self.setup_phase().unwrap(),
            Phase::Setup => self.transition_setup_to_action().unwrap(),
            Phase::Research => self.research_phase(),
            Phase::Action => (), //TODO
            Phase::Production => self.production_phase(),
        }
    }

    fn setup_phase(&mut self) -> undo::Result {
        let player_ids = self.get_state().players.iter().map(|p| p.id).collect::<Vec<usize>>();
        let mut queue = self.record.queue();
        for id in player_ids {
            // assign corporations
            queue.apply(DrawCards{player_id: id, count: 2, card_type: CardType::Corporation});
            // assign start cards
            queue.apply(DrawCards{player_id: id, count: 10, card_type: CardType::Project});
        }
        match queue.commit() {
            Ok(()) => self.record.as_mut_target().phase = Phase::Setup,
            Err(err) => return Err(err),
        };
        Ok(())
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
            self.record.apply(DrawCards{player_id: id, count: 10, card_type: CardType::Project}).unwrap();
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

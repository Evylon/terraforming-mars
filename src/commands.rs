use undo::Command;
use std::{error::Error, fmt};

use crate::player::Resource;
use crate::game_state::{GameState, OwnedCard, Phase};
use crate::card::{Card, CardType, Tags};

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

fn check_requirements(card: &Card, player_id: usize, game_state: &GameState) -> undo::Result {
    if game_state.temperature < card.requirements.global.min_temperature ||
        game_state.temperature > card.requirements.global.max_temperature {
            return Err(Box::new(CannotExecute{
                reason: format!("Requirements not met! Temperature {} <= {} <= {} not satisfied!",
                        card.requirements.global.min_temperature, game_state.temperature, card.requirements.global.max_temperature)
            }))
    }
    if game_state.oxygen < card.requirements.global.min_oxygen ||
        game_state.oxygen > card.requirements.global.max_oxygen {
            return Err(Box::new(CannotExecute{
                reason: format!("Requirements not met! Oxygen {} <= {} <= {} not satisfied!",
                        card.requirements.global.min_oxygen, game_state.oxygen, card.requirements.global.max_oxygen)
            }))
    }
    if game_state.oceans_placed < card.requirements.global.min_ocean ||
        game_state.oceans_placed > card.requirements.global.min_ocean {
            return Err(Box::new(CannotExecute{
                reason: format!("Requirements not met! Oceans {} <= {} <= {} not satisfied!",
                        card.requirements.global.min_ocean, game_state.oceans_placed, card.requirements.global.max_ocean)
            }))
    }
    let mut owned_tags: Vec<&Tags> = game_state.cards_in_play.iter().filter(|c| c.owner == player_id).map(|c| &c.card.tags).flatten().collect();
    // check requirements by removing ("counting") tags from owned tags if they are required by the card
    for tag in card.requirements.local.iter() {
        match owned_tags.iter().position(|t| **t == *tag) {
            Some(idx) => {
                owned_tags.remove(idx);
            },
            None => return Err(Box::new(CannotExecute{reason: format!("Requirements not met! Player {} hat not enough {} tags!", player_id, tag)})),
        }
    }
    Ok(())
}

impl Command<GameState> for PlayCard {
    fn apply(&mut self, game_state: &mut GameState) -> undo::Result {
        let player = game_state.get_player_mut(self.owner_id);
        let card = match player.hand.iter().position(|c| c.id == self.card_id) {
            Some(idx) => player.hand.remove(idx),
            None => return Err(Box::new(CannotExecute{reason: format!("Card {} not found in player {}'s hand!", self.card_id, self.owner_id)})),
        };
        // TODO allow to substitute megecredits with steel and titanium
        if player.inventory.megacredits < card.cost {
            return Err(Box::new(CannotExecute{
                reason: format!("Insufficient funds! Player {} need {} Megacredits to play card {}!", self.owner_id, card.cost, card.id)
            }))
        } else {
            player.inventory.megacredits -= card.cost;
        }
        check_requirements(&card, self.owner_id, game_state)?;
        // TODO check if actions on card can be executed (i.e. remove resources from other player)
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

pub struct DiscardResearch{pub player_id: usize, pub card_ids: Vec<String>}

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
}

pub enum CmdWrapper {
    ModResources(ModResources),
    DrawCards(DrawCards),
    PlayCard(PlayCard),
    ResearchCards(ResearchCards),
    ChooseCorporation(ChooseCorporation),
}

#[derive(Debug)]
pub struct CannotExecute{pub reason: String}

impl fmt::Display for CannotExecute {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.reason)
    }
}
impl Error for CannotExecute {}

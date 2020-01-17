use undo::Command;

use crate::player::{Player, Resource};
use crate::game_state::GameState;
use crate::card::Card;

pub struct DrawCard{pub player_id: usize, pub count: usize}

impl Command<GameState> for DrawCard {
    // pile: [c1, c2, c3] -> hand: [c3, c2, c1]
    // draw_cards 'reverses' the card order. the last card in the vector is the last one drawn
    fn apply(&mut self, game_state: &mut GameState) -> undo::Result {
        let mut cards = game_state.project_pile.draw_cards(self.count);
        let player = game_state.get_player(self.player_id);
        player.hand.append(cards.as_mut());
        Ok(())
    }
    // hand: [c3, c2, c1] -> pile: [c1, c2, c3]
    fn undo(&mut self, game_state: &mut GameState) -> undo::Result {
        let player = game_state.get_player(self.player_id);
        let draw_range = player.hand.len()-self.count..;
        let mut cards = player.hand.drain(draw_range).rev().collect::<Vec<Card>>();
        game_state.project_pile.draw_pile.append(cards.as_mut());
        Ok(())
    }
}

pub struct AddResources{pub player_id: usize, pub rescs: Vec::<Resource>}

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

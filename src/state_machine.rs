use undo::{Command, Record, Chain};

use crate::player::ActionState;
use crate::game_state::{GameState, Phase};
use crate::card::{Card, CardType, Resource};
use crate::commands::*;


pub struct StateMachine {
    record: Record<GameState>,
    cards: Vec<Card>,
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
            return CannotExecute::new("Cannot advance to Action phase, a player has not selected a corporation!".to_owned());
        }
        // players may hold only projects, no corporations
        if self.get_state().players.iter().flat_map(|p| &p.hand).any(|card| card.card_type == CardType::Corporation) {
            return CannotExecute::new("Cannot advance to Action phase, a player has a corporation card in hand!".to_owned());
        }
        // all players have to empty their research queue
        if !self.get_state().players.iter().all(|p| p.research_queue.is_empty()) {
            return CannotExecute::new("Cannot advance to Action phase, a player still has research enqueued!".to_owned());
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

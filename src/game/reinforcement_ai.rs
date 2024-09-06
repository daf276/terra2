use crate::game::game_state::Status::{Loss, Win};
use crate::game::game_state::{Action, GameState};
use rurel::mdp::{Agent, State};
use std::cmp::min;

pub struct MyAgent {
    pub(crate) state: GameState,
}
impl Agent<GameState> for MyAgent {
    fn current_state(&self) -> &GameState {
        &self.state
    }
    fn take_action(&mut self, action: &Action) -> () {
        self.state.advance(action.to_owned());
    }
}

impl State for GameState {
    type A = Action;

    fn reward(&self) -> f64 {
        if self.legal_actions.is_empty() {
            return -1000f64;
        };
        match self {
            GameState { status: Win, .. } => 1000f64,
            GameState { status: Loss, .. } => -1000f64,
            _ => {
                min(15, self.resources.education_culture) as f64
                    + min(15, self.resources.tech_economy) as f64
                    + min(15, self.resources.sustainability) as f64
                //- (0.5 * self.resources.instant_co2 as f64)
            }
        }
    }
    fn actions(&self) -> Vec<Action> {
        self.legal_actions.clone()
    }
}

use crate::game::game_state::{Action, GameState};
use rurel::mdp::Agent;

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

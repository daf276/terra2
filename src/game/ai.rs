use crate::game::buildings::Building;
use crate::game::game_state::Action::BuildInfrastructure;
use crate::game::game_state::Status::{Loss, Win};
use crate::game::game_state::{find_legal_actions, Action, GameState};
use rayon::prelude::*;
use rustc_hash::{FxHashMap, FxHasher};
use std::cmp::min;
use std::hash::{Hash, Hasher};
use std::sync::{Arc, Mutex};

pub fn evaluate_gamestate(state: &GameState) -> i16 {
    match state {
        GameState { status: Win, .. } => 1000,
        GameState { status: Loss, .. } => -1000,
        _ => {
            min(15, state.resources.education_culture)
                + min(15, state.resources.tech_economy)
                + min(15, state.resources.sustainability)
        }
    }
}
pub fn search_best_move(
    depth: u16,
    state: &GameState,
    transposition_table: Arc<Mutex<FxHashMap<u64, i16>>>,
) -> (i16, Action) {
    if depth == 0 {
        return (evaluate_gamestate(state), Action::Terraform(420));
    }

    // Check if the evaluation is already cached
    let state_hash = hash_state(state);
    {
        let table = transposition_table.lock().unwrap();
        if let Some(&cached_eval) = table.get(&state_hash) {
            return (cached_eval, Action::Terraform(420));
        }
    }

    let mut actions: &Vec<Action> = &state
        .legal_actions
        .clone()
        .into_iter()
        .filter(|&a| match a {
            BuildInfrastructure(_, _) => false,
            _ => true,
        })
        .collect();
    if actions.is_empty() {
        actions = &state.legal_actions;
    }
    let max_eval = actions
        .par_iter()
        .map(|action| {
            let mut new_state = state.clone();
            new_state.advance(*action);
            let (eval, _) = search_best_move(depth - 1, &new_state, Arc::clone(&transposition_table));
            (eval, action.clone())
        })
        .max_by(|(eval, _), (eval2, _)| eval.cmp(eval2))
        .unwrap_or((-1000, Action::Terraform(420)));

    {
        let mut table = transposition_table.lock().unwrap();
        table.insert(state_hash, max_eval.0); // Cache the evaluation
    }
    max_eval
}

fn hash_state(state: &GameState) -> u64 {
    let mut hasher = FxHasher::default();
    state
        .tiles
        .iter()
        .map(|t| t.usable)
        .collect::<Vec<bool>>()
        .hash(&mut hasher);
    state
        .tiles
        .iter()
        .map(|t| t.spaces)
        .collect::<Vec<[Building; 3]>>()
        .hash(&mut hasher);
    state.resources.hash(&mut hasher);
    state.season.hash(&mut hasher);
    state.doom_timer.hash(&mut hasher);
    hasher.finish()
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::Bencher;

    use super::*;
    #[bench]
    fn bench_search_best_move(b: &mut Bencher) {
        let state = GameState::initialize();

        b.iter(|| {
            test::black_box({
                let transposition_table = Arc::new(Mutex::new(FxHashMap::default()));
                let (eval, best_move) = search_best_move(5, &state.clone(), Arc::clone(&transposition_table));
            });
        });
    }
}

#![feature(trivial_bounds)]
#![feature(test)]

use crate::game::ai::search_best_move;
use crate::game::game_state::Status::{Loss, Running, Win};
use crate::game::game_state::{Action, GameState};
use rustc_hash::FxHashMap;
use std::collections::HashMap;
use std::io;
use std::time::Instant;

pub mod game;

//static mut transposition_table: Option<FxHashMap<u64, i16>> = None;

fn main() {
    /*//let file = File::open("saved.json").unwrap();
    //let map: HashMap<GameState, HashMap<Action, f64>> = serde_json::from_reader(&file).unwrap();
    let mut trainer = AgentTrainer::new();
    let mut training_game_state = GameState::initialize();
    //trainer.import_state(map);

    for i in (0..10000) {
        let mut agent = MyAgent { state: training_game_state.clone() };
        trainer.train(&mut agent,
                      &QLearning::new(0.2, 0.1, 0.0),
                      &mut SinkStates {},
                      &RandomExploration::new());
    }

    //let file = File::create("saved.json").unwrap();
    //serde_json::to_writer(file, &trainer.export_learned_values()).unwrap();

    let mut game_state = training_game_state.clone();
    for action in &game_state.legal_actions {
        let eval = trainer.expected_value(&game_state, action);
        println!("Action: {action:?}, Eval: {eval:?}")
    }*/
    let mut game_state = GameState::initialize();
    let mut input_string = String::new();

    game_state.tiles.iter().for_each(|i| {
        let spaces = i.spaces;
        println!(
            "Landscape {:?}, Spaces: {:?}, Connections {:?}",
            i.landscape, spaces, i.connections
        )
    });
    println!(
        "Co2: {}, Tech/Econ: {}, Sustainability: {}, Edu/Cult: {}, Co2 per Year: {}",
        &game_state.resources.instant_co2,
        &game_state.resources.tech_economy,
        &game_state.resources.sustainability,
        &game_state.resources.education_culture,
        &game_state.resources.yearly_co2,
    );
    for i in 0..game_state.legal_actions.len() {
        println!("{}. {:?}", i, game_state.legal_actions[i])
    }

    unsafe {
        while input_string.trim() != "x" && game_state.status == Running {
            let now = Instant::now();
            //transposition_table = Some(FxHashMap::default());
            let (eval, best_move) = search_best_move(5, &game_state);
            println!("Best move: {best_move:?}, Eval: {eval:?}");
            let elapsed = now.elapsed();
            println!("Elapsed: {:.2?}", elapsed);
            let action = parse_input(&game_state, &mut input_string);
            game_state.advance(action);
            print_tiles(&game_state);
            print_resources(&game_state);
            print_legal_actions(&game_state);
        }
    }

    if game_state.status == Win {
        println!("You won!")
    }
    if game_state.status == Loss {
        println!("You lose!")
    }
    println!("See you later!");
}

fn parse_input(game_state: &GameState, mut input_string: &mut String) -> Action {
    input_string.clear();
    io::stdin().read_line(&mut input_string).unwrap();
    let input: &str = input_string.trim();
    let action_index = input.parse::<usize>().unwrap();
    game_state.legal_actions[action_index]
}

fn print_tiles(game_state: &GameState) {
    game_state.tiles.iter().for_each(|i| {
        let spaces = i.spaces;
        println!(
            "Landscape {:?}, Spaces: {:?}, Connections {:?}",
            i.landscape, spaces, i.connections
        )
    });
}

fn print_resources(game_state: &GameState) {
    println!(
        "Co2: {}, Tech/Econ: {}, Sustainability: {}, Edu/Cult: {}, Co2 per Year: {}",
        &game_state.resources.instant_co2,
        &game_state.resources.tech_economy,
        &game_state.resources.sustainability,
        &game_state.resources.education_culture,
        &game_state.resources.yearly_co2,
    );
}

fn print_legal_actions(game_state: &GameState) {
    for i in 0..game_state.legal_actions.len() {
        println!("{}. {:?}", i, game_state.legal_actions[i])
    }
}

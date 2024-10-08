use crate::game::buildings::Building;
use crate::game::game_state::Action::{Build, Terraform};
use crate::game::game_state::Season::Spring;
use crate::game::game_state::Status::{Loss, Running, Win};
use crate::game::resources::Resources;
use crate::game::tile::Landscape::*;
use crate::game::tile::{filter_actual_connections, Landscape, Tile, POSSIBLE_CONNECTIONS};
use rand::prelude::IteratorRandom;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::IntoEnumIterator;
use Action::BuildInfrastructure;
use Season::{Autumn, Summer, Winter};

const MAP_SIZE: usize = 13;

#[derive(Clone, Hash, PartialEq, Debug, Serialize, Deserialize, Eq)]
pub struct GameState {
    pub tiles: [Tile; MAP_SIZE],
    pub resources: Resources,
    pub doom_timer: u8,
    pub season: Season,
    pub legal_actions: Vec<Action>,
    pub status: Status,
}

#[derive(Copy, Clone, Debug, PartialEq, Hash, Serialize, Deserialize, Eq)]
pub enum Status {
    Running,
    Win,
    Loss,
}
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum Action {
    Build(Building, usize),
    BuildInfrastructure(usize, usize),
    Terraform(usize),
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Serialize, Deserialize, Eq)]
#[repr(u8)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

impl GameState {
    pub fn initialize() -> GameState {
        let mut rng = rand::thread_rng();
        let mut tileset = create_tileset();
        let mut tiles: [Tile; MAP_SIZE] = [Tile::empty(Plains); 13];

        for i in 0..tiles.len() {
            if i == 6 {
                continue;
            }
            let (&landscape, &mut mut number_left) = tileset
                .iter_mut()
                .filter(|(_, &mut number_left)| number_left > 0)
                .choose(&mut rng)
                .unwrap();
            number_left -= 1;
            tiles[i] = Tile::empty(landscape);
        }
        tiles[6].usable = true;

        GameState {
            tiles,
            resources: Resources::new(0, 0, 0, 0, 0),
            doom_timer: 0,
            legal_actions: find_legal_actions(&tiles, 0),
            season: Spring,
            status: Running,
        }
    }

    pub fn advance(&mut self, action: Action) {
        match action {
            Build(building, tile) => self.build(building, tile),
            BuildInfrastructure(from, to) => self.build_infrastructure(from, to),
            Terraform(tile) => self.terraform(tile),
        }
        //println!("Took action {:?}", &action);
        self.check_loss_condition();
        self.check_win_condition();
        self.advance_season();
        self.legal_actions = find_legal_actions(&self.tiles, self.resources.education_culture);
    }

    fn build(&mut self, building: Building, tile_to_build_on: usize) {
        let tile = &mut self.tiles[tile_to_build_on];
        tile.build(building);
        self.resources += building.cost(*tile);
    }

    fn terraform(&mut self, tile: usize) {
        self.tiles[tile].terraform();
        self.resources += Resources::new(3, 0, -3, 0, 0);
    }

    fn build_infrastructure(&mut self, tile_from: usize, tile_to: usize) {
        self.tiles[tile_from].connect(tile_to);
        self.tiles[tile_to].connect(tile_from);
        self.tiles[tile_to].usable = true;
        self.resources += Resources::new(2, 0, -3, 0, 0);
    }

    fn advance_season(&mut self) {
        match self.season {
            Spring => self.season = Summer,
            Summer => self.season = Autumn,
            Autumn => self.season = Winter,
            Winter => {
                self.season = Spring;
                self.resources.instant_co2 += self.resources.yearly_co2;
            }
        }
    }

    fn check_loss_condition(&mut self) {
        if self.doom_timer == 4 {
            self.status = Loss;
        } else if self.resources.instant_co2 >= 20 {
            self.doom_timer += 1;
        } else if self.resources.instant_co2 < 20 {
            self.doom_timer = 0;
        }
    }

    fn check_win_condition(&mut self) {
        if self.resources.sustainability >= 15
            && self.resources.education_culture >= 15
            && self.resources.tech_economy >= 15
        {
            self.status = Win;
        }
    }
}

pub fn find_legal_actions(tiles: &[Tile; MAP_SIZE], science: i16) -> Vec<Action> {
    let mut actions = Vec::new();

    for (index, &tile) in tiles.iter().enumerate().filter(|(_, t)| t.usable) {
        // Check for terraforming actions
        if tiles[index].landscape != Plains && tiles[index].landscape != Ocean {
            actions.push(Terraform(index));
        }

        // Check for infrastructure actions
        for &possible in &POSSIBLE_CONNECTIONS[index] {
            if filter_actual_connections(tiles, possible) {
                actions.push(possible);
            }
        }

        // Check for build actions
        for building in Building::iter() {
            if building.can_build_on_tile(&tiles[index]) && building.has_enough_science(science) {
                actions.push(Build(building, index));
            }
        }
    }

    actions
}

fn create_tileset() -> HashMap<Landscape, i32> {
    let mut tileset = HashMap::new();
    tileset.insert(Mountain, 3);
    tileset.insert(Plains, 3);
    tileset.insert(Forest, 3);
    tileset.insert(Desert, 3);
    tileset.insert(Ocean, 3);
    tileset.insert(Swamp, 3);
    tileset
}

#[cfg(test)]
mod tests {
    extern crate test;

    use test::Bencher;

    use super::*;
    #[bench]
    fn bench_find_legal_actions(b: &mut Bencher) {
        let state = GameState::initialize();

        b.iter(|| {
            test::black_box(find_legal_actions(&state.tiles, state.resources.tech_economy));
        });
    }

    #[bench]
    fn bench_advance_build(b: &mut Bencher) {
        let state = GameState::initialize();
        let first_build_action = *state.legal_actions.iter().find(|&&a| matches!(a, Build(_, _))).unwrap();

        b.iter(|| test::black_box(state.clone().advance(first_build_action)));
    }

    #[bench]
    fn bench_advance_build_connection(b: &mut Bencher) {
        let state = GameState::initialize();

        let first_action = *state
            .legal_actions
            .iter()
            .find(|&&a| matches!(a, BuildInfrastructure(_, _)))
            .unwrap();

        b.iter(|| {
            test::black_box(state.clone().advance(first_action));
        });
    }

    #[bench]
    fn bench_gamestate_clone(b: &mut test::Bencher) {
        let state = GameState::initialize();
        b.iter(|| test::black_box(state.clone()))
    }
}

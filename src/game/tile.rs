use crate::game::buildings::Building;
use crate::game::buildings::Building::{Empty, EnvironmentalProtectionArea, NationalPark};
use crate::game::game_state::Action;
use crate::game::game_state::Action::BuildInfrastructure;
use crate::game::tile::Landscape::Plains;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

lazy_static! {
    pub static ref POSSIBLE_CONNECTIONS: Vec<Vec<Action>> = vec![
        vec![BuildInfrastructure(0, 2)],
        vec![BuildInfrastructure(1, 4)],
        vec![
            BuildInfrastructure(2, 0),
            BuildInfrastructure(2, 4),
            BuildInfrastructure(2, 5),
            BuildInfrastructure(2, 6),
        ],
        vec![BuildInfrastructure(3, 5)],
        vec![
            BuildInfrastructure(4, 1),
            BuildInfrastructure(4, 2),
            BuildInfrastructure(4, 6),
            BuildInfrastructure(4, 7),
        ],
        vec![
            BuildInfrastructure(5, 2),
            BuildInfrastructure(5, 3),
            BuildInfrastructure(5, 6),
            BuildInfrastructure(5, 8),
        ],
        vec![
            BuildInfrastructure(6, 2),
            BuildInfrastructure(6, 4),
            BuildInfrastructure(6, 5),
            BuildInfrastructure(6, 7),
            BuildInfrastructure(6, 8),
            BuildInfrastructure(6, 10),
        ],
        vec![
            BuildInfrastructure(7, 4),
            BuildInfrastructure(7, 6),
            BuildInfrastructure(7, 9),
            BuildInfrastructure(7, 10),
        ],
        vec![
            BuildInfrastructure(8, 4),
            BuildInfrastructure(8, 6),
            BuildInfrastructure(8, 10),
            BuildInfrastructure(8, 11),
        ],
        vec![BuildInfrastructure(9, 7)],
        vec![
            BuildInfrastructure(10, 6),
            BuildInfrastructure(10, 7),
            BuildInfrastructure(10, 8),
            BuildInfrastructure(10, 12),
        ],
        vec![BuildInfrastructure(11, 8)],
        vec![BuildInfrastructure(12, 10)],
    ];
}
#[derive(Copy, Clone, Hash, Eq, PartialEq, Debug, Serialize, Deserialize)]
pub struct Tile {
    pub spaces: [Building; 3],
    pub connections: [bool; 13],
    pub landscape: Landscape,
    pub spaces_left: u8,
}
#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug, Serialize, Deserialize)]
#[repr(u8)]
pub enum Landscape {
    Plains,
    Ocean,
    Mountain,
    Swamp,
    Desert,
    Forest,
}

impl Tile {
    pub fn empty(landscape: Landscape) -> Tile {
        Tile {
            landscape,
            spaces: [Empty; 3],
            spaces_left: 3,
            connections: [false; 13],
        }
    }

    pub fn connect(&mut self, tile_to: usize) {
        self.connections[tile_to] = true;
    }

    pub fn build(&mut self, building: Building) {
        if building == EnvironmentalProtectionArea {
            self.spaces[0] = building;
            self.spaces[1] = building;
            self.spaces[2] = building;
            self.spaces_left -= 3;
        } else if building == NationalPark {
            self.spaces[0] = building;
            self.spaces[1] = building;
            self.spaces_left -= 2;
        } else {
            if self.spaces_left == 3 {
                self.spaces[0] = building;
            } else if self.spaces_left == 2 {
                self.spaces[1] = building;
            } else if self.spaces_left == 1 {
                self.spaces[2] = building;
            }
            self.spaces_left -= 1;
        }
    }

    pub fn terraform(&mut self) {
        self.landscape = Plains;
    }
}

pub fn filter_actual_connections(tiles: [Tile; 13], possible_connection: Action) -> bool {
    match possible_connection {
        BuildInfrastructure(from, to) => !tiles[from].connections[to],
        _ => false,
    }
}

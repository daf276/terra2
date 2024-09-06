use crate::game::buildings::Building::{
    CoalPowerPlant, EnvironmentalProtectionArea, Livestock, OffshoreTurbines, River, SolarPark, Trees, Zoo,
};
use crate::game::resources::Resources;
use crate::game::tile::{Landscape, Tile};
use serde::{Deserialize, Serialize};
use strum_macros::{EnumIter, EnumString};
use Building::{NationalPark, University};

#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug, EnumString, EnumIter, Serialize, Deserialize)]
#[repr(u8)]
pub enum Building {
    Factory,
    Store,
    CoalPowerPlant,
    Trees,
    River,
    Livestock,
    Field,
    SolarPark,
    OffshoreTurbines,
    Biotope,
    NationalPark,
    EnvironmentalProtectionArea,
    School,
    Museum,
    Zoo,
    Library,
    University,
    Empty,
}

impl Building {
    pub fn cost(self, build_tile: Tile) -> Resources {
        match self {
            Building::Factory => adjusted_coal_power_plant_cost(build_tile),
            Building::Store => Resources::new(1, 2, -1, 0, 0),
            Building::CoalPowerPlant => Resources::new(6, 5, -4, 0, 2),
            Building::Trees => adjusted_tree_cost(build_tile),
            Building::River => adjusted_river_cost(build_tile),
            Building::Livestock => Resources::new(4, 1, 2, 0, 0),
            Building::Field => Resources::new(0, -1, 2, 0, 0),
            Building::SolarPark => Resources::new(4, 1, 2, 0, -2),
            Building::OffshoreTurbines => Resources::new(6, 1, 4, 0, -3),
            Building::Biotope => Resources::new(-3, 0, 2, 0, 0),
            Building::NationalPark => Resources::new(-6, 0, 4, 0, 0),
            Building::EnvironmentalProtectionArea => Resources::new(-9, 0, 6, 0, 0),
            Building::School => Resources::new(5, 0, -1, 5, 0),
            Building::Museum => Resources::new(3, 0, 0, 4, 0),
            Building::Zoo => adjusted_zoo_cost(build_tile),
            Building::Library => Resources::new(3, 1, 0, 3, 0),
            University => Resources::new(6, 3, 0, 4, 0),
            Building::Empty => Resources::new(0, 0, 0, 0, 0),
        }
    }

    pub fn can_build_on_tile(self, tile: Tile) -> bool {
        let spaces = tile.spaces;
        if spaces.contains(&self) {
            return false;
        };
        if tile.spaces_left < 3 && self == EnvironmentalProtectionArea {
            return false;
        };
        if tile.spaces_left < 2 && self == NationalPark {
            return false;
        };
        if tile.spaces_left < 1 {
            return false;
        };
        match tile.landscape {
            Landscape::Plains => self != OffshoreTurbines,
            Landscape::Ocean => self == OffshoreTurbines,
            Landscape::Mountain => self == CoalPowerPlant || self == Livestock || self == River || self == Trees,
            Landscape::Swamp => self == EnvironmentalProtectionArea,
            Landscape::Desert => self == SolarPark || self == EnvironmentalProtectionArea,
            Landscape::Forest => self == River || self == Trees || self == Zoo,
        }
    }

    pub fn has_enough_science(self, science: i16) -> bool {
        match self {
            SolarPark => science >= 10,
            OffshoreTurbines => science >= 15,
            University => science >= 10,
            _ => true,
        }
    }
}

fn adjusted_coal_power_plant_cost(build_tile: Tile) -> Resources {
    let cost = Resources::new(3, 4, -2, 0, 1);
    match build_tile.landscape {
        Landscape::Mountain => Resources {
            tech_economy: 7,
            ..cost
        },
        _ => cost,
    }
}

fn adjusted_tree_cost(build_tile: Tile) -> Resources {
    let cost = Resources::new(0, -2, 1, 0, -2);
    match build_tile.landscape {
        Landscape::Forest => Resources {
            sustainability: 2,
            ..cost
        },
        _ => cost,
    }
}

fn adjusted_river_cost(build_tile: Tile) -> Resources {
    let cost = Resources::new(0, -2, 2, 0, -1);
    match build_tile.landscape {
        Landscape::Mountain => Resources {
            sustainability: 3,
            ..cost
        },
        _ => cost,
    }
}

fn adjusted_zoo_cost(build_tile: Tile) -> Resources {
    let cost = Resources::new(4, 0, 3, 2, 0);
    match build_tile.landscape {
        Landscape::Mountain => Resources {
            sustainability: 4,
            ..cost
        },
        _ => cost,
    }
}

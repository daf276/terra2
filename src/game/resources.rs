use serde::{Deserialize, Serialize};
use std::ops::AddAssign;

#[derive(Copy, Clone, Hash, PartialEq, Debug, Serialize, Deserialize, Eq)]
pub struct Resources {
    pub(crate) instant_co2: i16,
    pub(crate) tech_economy: i16,
    pub(crate) sustainability: i16,
    pub(crate) education_culture: i16,
    pub(crate) yearly_co2: i16,
}

impl Resources {
    pub fn new(
        instant_co2: i16,
        tech_economy: i16,
        sustainability: i16,
        education_culture: i16,
        yearly_co2: i16,
    ) -> Resources {
        Resources {
            instant_co2,
            tech_economy,
            sustainability,
            education_culture,
            yearly_co2,
        }
    }
}

impl AddAssign for Resources {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            instant_co2: self.instant_co2 + rhs.instant_co2,
            tech_economy: self.tech_economy + rhs.tech_economy,
            yearly_co2: self.yearly_co2 + rhs.yearly_co2,
            sustainability: self.sustainability + rhs.sustainability,
            education_culture: self.education_culture + rhs.education_culture,
        };
    }
}

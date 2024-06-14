use serde_derive::{Serialize, Deserialize};
use eosim::global_properties::GlobalProperty;
use eosim::data_containers::PropertyWithDefault;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct InfectionProbabilities {
    pub child_to_child: f64,
    pub child_to_adult: f64,
    pub adult_to_child: f64,
    pub adult_to_adult: f64,
    pub adult_to_elderly: f64,
    pub elderly_to_adult: f64,
    pub child_to_elderly: f64,
    pub elderly_to_child: f64,
    pub elderly_to_elderly: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Parameters {
    pub population: usize,
    pub r0: f64,
    pub infectious_period: f64,
    pub initial_infections: usize,
    pub random_seed: u64,
    pub death_rate: f64,
    pub infection_probabilities: InfectionProbabilities,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Scenario {
    pub scenario: usize,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Config {
    Single(Parameters),
    Multiple(Vec<Parameters>),
}

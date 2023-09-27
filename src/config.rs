use serde_derive::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub proximity_threshold: f32,
    pub infection_rate: f32,
    pub zombie_speed_following: f32,
    pub zombie_speed: f32,
    pub zombies: i32,
    pub zombie_vision_range: f32,
}

pub fn load_config() -> Result<Config, toml::de::Error> {
    let contents = fs::read_to_string("simulation.toml").expect("Failed to read config.toml");
    toml::from_str(&contents)
}

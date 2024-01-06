use serde_derive::Deserialize;
use std::fs;
use std::env;

#[derive(Deserialize)]
pub struct Config {
    pub screen_width:f32,
    pub screen_height:f32,
    pub proximity_threshold: f32,
    pub infection_rate: f32,
    pub zombie_speed_following: f32,
    pub zombie_speed: f32,
    pub zombies: i32,
    pub zombie_vision_range: f32,
    pub humans: i32,
    pub humans_zombie_distance_to_start_run: f32,
    pub quadtree_size: usize,
    pub human_grouping_distance: f32,
    pub humans_speed_range_1: f32,
    pub humans_speed_range_2: f32,
}

pub fn load_config() -> Result<Config, toml::de::Error> {
    let args: Vec<String> = env::args().collect();
    let config_file = if args.len() > 1 {
        &args[1]
    } else {
        "simulation.toml"
    };

    let contents = fs::read_to_string(config_file).expect("Failed to read config.toml");
    toml::from_str(&contents)
}

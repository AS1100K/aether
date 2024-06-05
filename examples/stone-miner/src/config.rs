use serde::{Deserialize, Serialize};
use std::fs::read_to_string;
use azalea::WalkDirection;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub username: String,
    pub server: String,
    pub mode: Mode,
    pub email: Option<String>,
    pub checkpoints: [Checkpoint; 4],
    pub directions: [WalkDir; 4],
    pub initial_y_rot: f32,
    pub y_start: i32,
    pub y_end: i32,
}

pub type Checkpoint = [f64; 3];

#[derive(Serialize, Deserialize, Clone, Eq, PartialEq, Debug)]
pub enum Mode {
    Online,
    Offline,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub enum WalkDir {
    #[default]
    Forward,
    Backward,
    Left,
    Right
}

impl WalkDir {
    pub fn to_azalea_walk_direction(&self) -> WalkDirection {
        match self {
            WalkDir::Forward => WalkDirection::Forward,
            WalkDir::Backward => WalkDirection::Backward,
            WalkDir::Right => WalkDirection::Right,
            WalkDir::Left => WalkDirection::Left
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        let contents: String = read_to_string("config.json").expect("Unable to load config.json");
        let config: Config =
            serde_json::from_str(&contents.as_str()).expect("Unable to parse config.json");

        config
    }
}

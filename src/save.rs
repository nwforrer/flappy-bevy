use std::{env, fs};

use bevy::prelude::*;

use crate::{GameState, Score};

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Dead), save_score);
    }
}

fn save_score(score: Res<Score>) {
    if score.current <= score.high {
        return;
    }

    info!("new high score");

    match get_save_path() {
        Ok(save_path) => {
            fs::create_dir_all(save_path.clone()).expect("Failed to create save directory");

            info!("saving score to {save_path}");

            if let Err(e) = fs::write(format!("{save_path}/highscore"), score.current.to_string()) {
                error!("Failed to save score! {}", e);
            }
        }
        Err(e) => error!("Failed to save score! {}", e),
    }
}

pub fn get_high_score() -> u32 {
    if let Ok(save_path) = get_save_path() {
        info!("getting score from {save_path}");

        if let Ok(highscore) = fs::read_to_string(format!("{save_path}/highscore")) {
            if let Ok(highscore) = str::parse::<u32>(&highscore) {
                return highscore;
            }
        }
    }

    0
}

fn get_save_path() -> Result<String, &'static str> {
    let package_name = env!("CARGO_PKG_NAME");
    match env::consts::OS {
        "linux" => Ok(format!(
            "{}/.local/share/{package_name}/saves",
            env!("HOME")
        )),
        _ => Err("No save path available for platform"),
    }
}

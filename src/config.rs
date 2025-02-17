use std::{fs::{self, File}, io::Write};

use serde::{Deserialize, Serialize};

use crate::exercise::UnloadedExerciseData;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AppConfig {
    pub unloaded_exercises: Vec<UnloadedExerciseData>
}

pub fn load_config() -> AppConfig {
    let config_dir_path = dirs::config_dir();
    match config_dir_path {
        Some(config_dir_path) => {
            let config_path = config_dir_path.join("test_generator/config.json");
            let config_string = fs::read_to_string(config_path);
            match config_string {
                Ok(config_string) => {
                    match serde_json::from_str::<AppConfig>(&config_string) {
                        Ok(config) => config,
                        Err(err) => {
                            println!("Failed to deserialize config file contents. Err: {}", err);
                            Default::default()
                        },
                    }
                },
                Err(err) => {
                    println!("Failed to read a config file. Err: {}", err);
                    Default::default()
                },
            }
        },
        None => {
            println!("Failed to get the config directory path!");
            Default::default()
        },
    }
}

pub fn save_config(config: &AppConfig) {
    let config_dir_path = dirs::config_dir();
    match config_dir_path {
        Some(config_dir_path) => {
            let config_path = config_dir_path.join("test_generator/config.json");
            match serde_json::to_string(config) {
                Ok(config) => {
                    let _ = fs::remove_file(&config_path);
                    let _ = fs::create_dir_all(config_path.parent()
                        .expect("Failed to get the config file parent dir"));
                    match File::create_new(&config_path) {
                        Ok(mut file) => {
                            if let Err(err) = file.write_all(config.as_bytes()) {
                                println!("Failed to write to a file. Err: {}", err)
                            }
                        },
                        Err(err) => println!("Failed to create a new config file. Path: {}. Err: {}",
                            config_path.to_str().unwrap(), err),
                    }
                },
                Err(err) => println!("Failed to serialize the config. Err: {}", err),
            }
        },
        None => println!("Failed to get the config directory path!"),
    }
}

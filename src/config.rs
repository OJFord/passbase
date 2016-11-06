extern crate serde_json;

use ::std;
use ::std::fs::File;
use ::std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(rename="User")]
    user: Option<String>
}


fn config_file() -> PathBuf {
    std::env::home_dir()
        .expect("Failed to determine $HOME dir!")
        .join(".passbase")
}

fn set_config(config: &Config) {
    let mut fp = File::create(config_file());
    match fp {
        Ok(mut buf) => { serde_json::to_writer(&mut buf, config); },
        Err(why) => { panic!("Failed to write config file: {}", why) },
    }
}

fn get_config() -> Config {
    let mut fp = File::open(config_file());
    match fp {
        Ok(buf) => match serde_json::from_reader(buf) {
            Ok(config) => config,
            Err(why) => { panic!("Failed to parse config file: {}", why) },
        },
        Err(_) => {
            let config = Config {
                user: None
            };
            set_config(&config);
            return config;
        }
    }
}

pub fn get_user() -> Option<String> {
    get_config().user
}

pub fn set_user(user: &String) {
    let mut config = get_config();
    config.user = Some(user.clone());
    set_config(&config);
}

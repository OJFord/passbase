extern crate serde_json;

use ::std::env;
use ::std::fs::File;
use ::std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(rename="User")]
    user: Option<String>
}

pub const KBFS_DATA_DIR: &'static str = ".passbase";

fn config_file() -> PathBuf {
    env::home_dir()
        .expect("Failed to determine $HOME dir!")
        .join(&KBFS_DATA_DIR)
}

fn set_config(config: &Config) {
    let fp = File::create(config_file());
    match fp {
        Ok(mut buf) => {
            serde_json::to_writer(&mut buf, config)
                .expect("Failed to write config");
        },
        Err(why) => { panic!("Failed to write config: {}", why) },
    }
}

fn get_config() -> Config {
    let fp = File::open(config_file());
    match fp {
        Ok(buf) => {
            serde_json::from_reader(buf)
                .expect("Failed to parse config file")
        }
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

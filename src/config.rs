extern crate serde_json;
extern crate dirs;

use ::std::default::Default;
use ::std::fs::File;
use ::std::path::PathBuf;

#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(rename="User")]
    user: Option<String>
}

impl Default for Config {
    fn default() -> Config {
        Config {
            user: Default::default()
        }
    }
}

pub const KBFS_DATA_DIR: &'static str = ".passbase";

fn config_file() -> Result<PathBuf, serde_json::Error> {
    let path = dirs::home_dir()
        .expect("Failed to determine $HOME dir!")
        .join(&KBFS_DATA_DIR);
    if path.exists() {
        assert!(path.is_file());
    } else {
        let _ = File::create(&path)
            .map(|mut buf| serde_json::to_writer(&mut buf, &Config::default()))
            .expect("Failed to create file.");
    }
    Ok(path)
}

fn set_config(config: &Config) {
    let _ = ::std::fs::OpenOptions::new()
        .write(true)
        .open(config_file().unwrap())
        .map(|mut buf| serde_json::to_writer(&mut buf, config))
        .expect("Failed to write to config file.");
}

fn get_config() -> Result<Config, serde_json::Error> {
    serde_json::from_reader(File::open(config_file()?)?)
}

pub fn get_user() -> Result<String, String> {
    get_config()
        .map_err(|err| err.to_string())
        ?.user
        .ok_or("User not set.".to_owned())
}

pub fn set_user(user: &String) {
    let mut config = get_config().unwrap();
    config.user = Some(user.clone());
    set_config(&config);
}

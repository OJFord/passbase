extern crate serde_json;

use self::serde_json::{Map, Value};
use std;

pub fn get_user() -> String {
    std::process::Command::new("keybase")
        .arg("login")
        .spawn()
        .expect("Keybase auth failed");
    let output = std::process::Command::new("keybase")
        .arg("status")
        .arg("-j")
        .output()
        .unwrap();
    let status: Map<String, Value> =
        serde_json::from_str(&String::from_utf8_lossy(&output.stdout)).unwrap();

    return String::from(status.get("Username").unwrap().as_str().unwrap());
}

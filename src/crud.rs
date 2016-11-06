extern crate rand;

use ::std::fs;
use ::std::error::Error;
use ::std::io;
use ::std::io::prelude::*;
use ::std::path::Path;
use ::std::process;
use self::rand::Rng;


pub fn create(passbase_dir: &Path, tag: &str) {
    let file = passbase_dir.join(tag);
    assert!(fs::metadata(&file).is_err(), format!("{} already exists!", tag));
    let mut fp = fs::File::create(&file).unwrap();

    let pass: String = rand::thread_rng()
        .gen_ascii_chars()
        .take(128)
        .collect();

    match fp.write_all(pass.as_bytes()) {
        Err(why) => {
            fs::remove_file(&file);
            panic!("Failed to write new password: {}", why.description());
        },
        Ok(_) => {
            read(passbase_dir, tag);
        }
    }
}

pub fn list(passbase_dir: &Path) {
    for entry in fs::read_dir(passbase_dir).unwrap() {
        let entry = entry.unwrap();
        if !entry.path().is_dir() {
            println!("{}", entry.file_name().into_string().unwrap());
        }
    }
}

pub fn read(passbase_dir: &Path, tag: &str) {
    let mut fp = fs::File::open(passbase_dir.join(tag)).unwrap();
    let mut buf = String::new();
    fp.read_to_string(&mut buf);

    //TODO: display this then wip to keep history clear (use less?)
    println!("{tag}: {password}", tag=tag, password=buf);
}

pub fn change(passbase_dir: &Path, tag: &str) {
    let file = passbase_dir.join(tag);
    assert!(fs::metadata(&file).is_ok(), format!("{} doesn't exist!", tag));
    let mut fp = fs::OpenOptions::new()
        .write(true)
        .open(&file)
        .unwrap();

    let pass: String = rand::thread_rng()
        .gen_ascii_chars()
        .take(128)
        .collect();

    match fp.write_all(pass.as_bytes()) {
        Err(why) => {
            panic!("Failed to write new password: {}", why.description());
        },
        Ok(_) => {
            read(passbase_dir, tag);
        }
    }
}

pub fn remove(passbase_dir: &Path, tag: &str) {
    let file = passbase_dir.join(tag);
    println!("Are you sure, remove password for {tag} [y/N]? ", tag=tag);
    let mut answer = String::new();
    io::stdin().read_line(&mut answer);
    match answer.trim().as_ref() {
        "y" | "Y" => {
            fs::remove_file(&file);
        },
        _ => {
            println!("Not removing {tag}", tag=tag);
        },
    }
}

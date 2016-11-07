extern crate rand;

use ::std::fs;
use ::std::error::Error;
use ::std::io;
use ::std::io::prelude::*;
use ::std::path::Path;
use self::rand::Rng;


pub fn create(passbase_dir: &Path, tag: &str) {
    let file = passbase_dir.join(tag);
    assert!(!file.is_file(), format!("Password for {} already exists!", tag));
    let mut fp = fs::File::create(&file)
        .expect("Failed to create file");

    let pass: String = rand::thread_rng()
        .gen_ascii_chars()
        .take(128)
        .collect();

    match fp.write_all(pass.as_bytes()) {
        Err(why) => {
            fs::remove_file(&file)
                .expect("Failed to remove created file");
            panic!("Failed: {}", why.description());
        },
        Ok(_) => {
            read(passbase_dir, tag);
        }
    }
}

pub fn list(passbase_dir: &Path) {
    let mut tags: Vec<_> = fs::read_dir(&passbase_dir)
        .expect("Failed to read directory")
        .map(|tag| tag.unwrap())
        .collect();
    tags.sort_by_key(|tag| tag.path());

    for tag in tags {
        if !tag.path().is_dir() {
            println!("{}", tag.file_name().into_string().unwrap());
        }
    }
}

pub fn read(passbase_dir: &Path, tag: &str) {
    let file = passbase_dir.join(tag);
    assert!(file.is_file(), format!("No password exists for {}", tag));

    let mut fp = fs::File::open(passbase_dir.join(tag))
        .expect("Failed to open file");
    let mut buf = String::new();
    fp.read_to_string(&mut buf)
        .expect("Failed to read file");

    //TODO: display this then wip to keep history clear (use less?)
    println!("{tag}: {password}", tag=tag, password=buf);
}

pub fn change(passbase_dir: &Path, tag: &str) {
    let file = passbase_dir.join(tag);
    assert!(file.is_file(), format!("No password exists for {}", tag));

    let mut fp = fs::OpenOptions::new()
        .write(true)
        .open(&file)
        .unwrap();

    let pass: String = rand::thread_rng()
        .gen_ascii_chars()
        .take(128)
        .collect();

    fp.write_all(pass.as_bytes())
        .expect("Failed to write new password");
    read(passbase_dir, tag);
}

pub fn remove(passbase_dir: &Path, tag: &str) {
    let file = passbase_dir.join(tag);
    assert!(file.is_file(), format!("No password exists for {}", tag));
    println!("Are you sure, remove password for {tag} [y/N]? ", tag=tag);

    let mut answer = String::new();
    io::stdin().read_line(&mut answer)
        .expect("Failed to read from stdin");
    match answer.trim().as_ref() {
        "y" | "Y" => {
            fs::remove_file(&file)
                .expect("Failed to remove file");
        },
        _ => {
            println!("Not removing password for {tag}", tag=tag);
        },
    }
}

extern crate clap;
extern crate rand;
extern crate serde_json;

use clap::{App, AppSettings, Arg, SubCommand};
use rand::Rng;
use serde_json::{Map, Value};
use std::fs;
use std::error::Error;
use std::io::prelude::*;
use std::path::Path;
use std::process;


fn read(passbase_dir: &Path, tag: &str) {
    let mut fp = fs::File::open(passbase_dir.join(tag)).unwrap();
    let mut buf = String::new();
    fp.read_to_string(&mut buf);

    //TODO: display this then wip to keep history clear (use less?)
    println!("{tag}: {password}", tag=tag, password=buf);
}

fn list(passbase_dir: &Path) {
    for entry in fs::read_dir(passbase_dir).unwrap() {
        let entry = entry.unwrap();
        if !entry.path().is_dir() {
            println!("{}", entry.file_name().into_string().unwrap());
        }
    }
}

fn create(passbase_dir: &Path, tag: &str) {
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

fn change(passbase_dir: &Path, tag: &str) {
    println!("Changing password for {tag}", tag=tag);
}

fn remove(passbase_dir: &Path, tag: &str) {
    println!("Removing password for {tag}", tag=tag);
}

fn main() {
    let tag_arg = Arg::with_name("tag")
        .index(1)
        .required(true)
        .takes_value(true)
        .value_name("NAME");
    let app_matches = App::new("Passbase")
        .version("0.1")
        .author("Oliver Ford <me@ojford.com>")
        .about("Password generation & management integrated with Keybase")
        .arg(tag_arg.clone())
        .setting(AppSettings::SubcommandsNegateReqs)
        .subcommand(
            SubCommand::with_name("list")
                .visible_alias("ls")
        )
        .subcommand(
            SubCommand::with_name("read")
                .visible_alias("cat")
                .arg(tag_arg.clone())
        )
        .subcommand(
            SubCommand::with_name("create")
                .visible_alias("touch")
                .arg(tag_arg.clone())
        )
        .subcommand(
            SubCommand::with_name("change")
                .arg(tag_arg.clone())
        )
        .subcommand(
            SubCommand::with_name("remove")
                .visible_alias("rm")
                .arg(tag_arg.clone())
        )
        .get_matches();

    process::Command::new("keybase").arg("login").spawn().expect("Keybase auth failed");
    let output = process::Command::new("keybase").arg("status").arg("-j").output().unwrap();
    let kb_status: Map<String, Value> = serde_json::from_str(
        &String::from_utf8_lossy(&output.stdout)
    ).unwrap();
    let kb_user: &str = kb_status.get("Username").unwrap().as_str().unwrap();
    let passbase_dir = Path::new("/keybase/private").join(kb_user).join("passbase");

    //TODO: if passbase_dir does not exist: create it

    match app_matches.subcommand() {
        ("list", _)             => { list(&passbase_dir); },
        ("create", Some(args))  => { create(&passbase_dir, args.value_of("tag").unwrap()); },
        ("change", Some(args))  => { change(&passbase_dir, args.value_of("tag").unwrap()); },
        ("remove", Some(args))  => { remove(&passbase_dir, args.value_of("tag").unwrap()); },
        ("read", Some(args))    => { read(&passbase_dir, args.value_of("tag").unwrap()); },
        _                       => { read(&passbase_dir, &app_matches.value_of("tag").unwrap()); },
    }
}

#![feature(custom_attribute)]
#![feature(proc_macro)]

extern crate clap;
#[macro_use]
extern crate serde_derive;

mod config;
mod crud;
mod keybase;

use ::std::fs;
use ::std::path::Path;
use crud::*;
use self::clap::{App, AppSettings, Arg, ArgMatches, SubCommand};


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

    let user: String;
    if let Some(config_user) = config::get_user() {
        user = config_user;
    } else {
        user = keybase::get_user();
        config::set_user(&user);
    }

    let passbase_dir = Path::new("/keybase/private")
        .join(user)
        .join(".passbase");
    match passbase_dir.exists() {
        true => {
            assert!(passbase_dir.is_dir(), format!(
                "A file {} already exists in KBFS!",
                config::KBFS_DATA_DIR
            ))
        },
        false => {
            println!("Passbase directory does not exist in KBFS, creating...");
            fs::create_dir(&passbase_dir)
                .expect("Failed to create Passbase directory");
        },
    }

    fn tag<'a>(args: &'a ArgMatches) -> &'a str {
        return args.value_of("tag").unwrap();
    }
    match app_matches.subcommand() {
        ("list", _) => list(&passbase_dir),
        ("create", Some(args)) => create(&passbase_dir, tag(args)),
        ("change", Some(args)) => change(&passbase_dir, tag(args)),
        ("remove", Some(args)) => remove(&passbase_dir, tag(args)),
        ("read", Some(args)) => read(&passbase_dir, tag(args)),
        _ => read(&passbase_dir, tag(&app_matches)),
    }
}

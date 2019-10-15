#![feature(custom_attribute)]
#![feature(slice_patterns)]

extern crate clap;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;

mod config;
mod crud;
mod keybase;

use ::std::fs;
use ::std::collections::HashSet;
use ::std::iter::FromIterator;
use ::std::path::Path;
use crud::*;
use self::clap::{App, AppSettings, Arg, ArgMatches, SubCommand};

// These are the easily accessible special characters on a UK keyboard;
// which, in my testing, do not break 'double-tap select' (to copy easily).
const ACCEPTED_SPECIAL_CHARS: &'static str  = "~`!@£&*_+-=\\,./|?";

lazy_static! {
    static ref ACCEPTED_SPECIALS_HASH: HashSet<char> = {
        ACCEPTED_SPECIAL_CHARS.chars().collect()
    };
}

fn main() {
    let tag_arg = Arg::with_name("tag")
        .index(1)
        .required(true)
        .takes_value(true)
        .value_name("NAME");

    let len_arg = Arg::with_name("length")
        .help("Sets the number of characters in password")
        .short("n")
        .long("length")
        .takes_value(true)
        .default_value("128")
        .validator(validate_number);

    let no_sym_arg = Arg::with_name("no-specials")
        .help("Sets a strictly alphanumeric password")
        .short("X")
        .long("no-specials");

    let sym_arg = Arg::with_name("specials")
        .help("Provides a set of special chars to use")
        // Conflicts with option is incompatible with a default
        //.conflicts_with(no_sym_arg.name)
        .short("s")
        .long("specials")
        .takes_value(true)
        .default_value(ACCEPTED_SPECIAL_CHARS)
        .validator(validate_special_chars);

    let app_matches = App::new("Passbase")
        .version("0.1")
        .author("Oliver Ford <dev@ojford.com>")
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
                .arg(no_sym_arg.clone())
                .arg(sym_arg.clone())
                .arg(len_arg.clone())
                .arg(tag_arg.clone())
        )
        .subcommand(
            SubCommand::with_name("change")
                .help("Changes a password. Previous can be `recover`ed.")
                .arg(no_sym_arg.clone())
                .arg(sym_arg.clone())
                .arg(len_arg.clone())
                .arg(tag_arg.clone())
        )
        .subcommand(
            SubCommand::with_name("recover")
                .help("Recovers the previous version of a `change`d password")
                .arg(tag_arg.clone())
        )
        .subcommand(
            SubCommand::with_name("remove")
                .help("DELETES given password FOREVER. Cannot be `recover`ed")
                .visible_alias("rm")
                .arg(tag_arg.clone())
        )
        .get_matches();

    let user: String;
    if let Ok(config_user) = config::get_user() {
        user = config_user;
    } else {
        user = keybase::get_user();
        config::set_user(&user);
    }

    let passbase_dir = Path::new("/Volumes/Keybase/private")
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

    fn len<'a>(args: &'a ArgMatches) -> u16 {
        return args.value_of("length").unwrap().parse::<u16>().unwrap();
    }

    fn specials<'a>(args: &'a ArgMatches) -> &'a str {
        if args.is_present("no-specials") {
            return "";
        } else {
            return args.value_of("specials").unwrap();
        }
    }

    match app_matches.subcommand() {
        ("list", _) => list(&passbase_dir),
        ("create", Some(args)) => {
            create(&passbase_dir, tag(args), len(args), specials(args))
        },
        ("change", Some(args)) => {
            change(&passbase_dir, tag(args), len(args), specials(args))
        },
        ("recover", Some(args)) => {
            recover(&passbase_dir, tag(args))
        },
        ("remove", Some(args)) => {
            remove(&passbase_dir, tag(args))
        },
        ("read", Some(args)) => {
            read(&passbase_dir, tag(args))
        },
        _ => {
            read(&passbase_dir, tag(&app_matches))
        },
    }
}

fn validate_special_chars(v: String) -> Result<(), String> {
    let given: HashSet<char> = v.chars().collect();

    if given.is_subset(&ACCEPTED_SPECIALS_HASH) {
        Ok(())
    } else {
        let intersection_of_requirements: HashSet<char> = given
            .intersection(&ACCEPTED_SPECIALS_HASH)
            .cloned()
            .collect();

        Err(format!(
            "must be a subset of {} -- use: {}",
            ACCEPTED_SPECIAL_CHARS,
            String::from_iter(
                intersection_of_requirements.iter().map(|c| *c)
            )
        ))
    }
}

fn validate_number(v: String) -> Result<(), String> {
    match v.parse::<u16>() {
        Ok(_) => Ok(()),
        Err(_) => Err(String::from("must be an integer")),
    }
}

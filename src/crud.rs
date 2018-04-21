extern crate rand;

use ::std::fs;
use ::std::error::Error;
use ::std::io;
use ::std::io::prelude::*;
use ::std::path::Path;
use ::std::process::Command;
use self::rand::Rng;

fn gen(len: u16, specials: &str) -> String {
    let mut pswd = String::new();

    let mut rng_a = rand::thread_rng();
    let mut rng_b = rand::thread_rng();

    let mut alphanums = rng_a.gen_ascii_chars();

    if specials.is_empty() {
        return alphanums.take(len as usize).collect();
    }

    let specials: Vec<char> = specials
        .chars()
        .map(|c| c.clone())
        .collect();

    for _ in 0..len {
        if rng_b.gen_weighted_bool(8) {
            let special: char = *rng_b.choose(specials.as_slice()).unwrap();
            pswd.push(special);
        } else {
            pswd.push(alphanums.next().unwrap());
        }
    }
    pswd
}

pub fn create(passbase_dir: &Path, tag: &str, len: u16, specials: &str) {
    let file = passbase_dir.join(tag);
    assert!(!file.is_file(), format!("Password for {} already exists!", tag));
    let mut fp = fs::File::create(&file)
        .expect("Failed to create file");

    match fp.write_all(gen(len, specials).as_bytes()) {
        Err(why) => {
            fs::remove_file(&file)
                .expect("Failed to remove created file");
            panic!("Failed: {}", why.description());
        },
        Ok(_) => {
            read(passbase_dir, tag, None);
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

pub fn read(passbase_dir: &Path, tag: &str, positions: Option<Vec<u16>>) {
    let file = passbase_dir.join(tag);
    assert!(file.is_file(), format!("No password exists for {}", tag));

    let ro_file = "/tmp/passbase-read";

    if positions {
        let characters = fs::read(file).expect("Unable to read file");
        let mut out = String::new();

        for pos in positions {
            out.write(pos + ":" + characters[pos] + "\t");
        }
        fs::write(ro_file, out.as_bytes());
    } else {
	fs::copy(file, ro_file)
	    .expect("Failed to access the filesystem");
    }

    let less = Command::new("less")
        .arg(ro_file)
        .spawn()
        .expect("Failed to spawn less");

    let exit = less.wait_with_output()
        .expect("Failed to wait on less")
        .status;

    assert!(exit.success());
}

pub fn change(passbase_dir: &Path, tag: &str, len: u16, specials: &str) {
    let file = passbase_dir.join(tag);
    assert!(file.is_file(), format!("No password exists for {}", tag));

    let mut old_file = file.clone();
    old_file.set_extension("old");

    match fs::rename(file, old_file) {
        Ok(_) => create(passbase_dir, tag, len, specials),
        Err(e) => panic!("Failed to rename old file: {}", e),
    }
}

pub fn recover(passbase_dir: &Path, tag: &str) {
    read(passbase_dir, format!("{}.old", tag).as_str(), None);
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

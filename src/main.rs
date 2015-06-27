#[macro_use]
extern crate horrorshow;
extern crate gazetta;
extern crate chrono;
extern crate clap;

#[macro_use]
extern crate lazy_static;

use std::{fs, io, process};
use std::process::Command;
use std::io::Write;
use std::env;
use std::path::{Path, PathBuf};
use std::fs::File;

use clap::{Arg, App, SubCommand};
use gazetta::prelude::*;

mod link;
mod person;
mod meta;
mod renderer;
use renderer::MyGazetta;

use chrono::offset::local::Local as Date;

macro_rules! try_exit {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => bail!("{}", e),
        }
    }
}

macro_rules! bail {
    ($($toks:tt)*) => {{
        let stderr = io::stderr();
        let mut stderr = stderr.lock();
        let _ = writeln!(stderr, $($toks)*);
        process::exit(1)
    }}
}

fn slugify(s: &str) -> String {
    let mut output = String::with_capacity(s.len());
    for c in s.trim().chars() {
        match c {
            '_'|'-'|'a'...'z'|'0'...'9' => output.push(c),
            'A'...'Z' => output.push(((c as u8)-('A' as u8)+('a' as u8)) as char), // Poor mans ASCII case conversion.
            _ if c.is_whitespace() && !output.ends_with("-") => output.push('-'),
            _ => {},
        }
    }
    output
}

fn main() {
    let name = &env::args().next().unwrap();
    let matches = App::new(&name)
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("SOURCE")
             .short("s")
             .long("source")
             .takes_value(true)
             .help("Specify the source directory (defaults to the current directory)"))
        .subcommand(SubCommand::new("render")
                    .about("Render the website")
                    .arg(Arg::with_name("FORCE")
                         .short("f")
                         .long("force")
                         .help("Overwrite any existing DEST."))
                    .arg(Arg::with_name("DEST")
                         .required(true)
                         .index(1)
                         .help("Destination directory")))
        .subcommand(SubCommand::new("new")
                    .about("Create a new page")
                    .arg(Arg::with_name("EDIT")
                         .short("e")
                         .long("edit")
                         .help("Edit new page in your $EDITOR"))
                    .arg(Arg::with_name("WHERE")
                         .required(true)
                         .index(1)
                         .help("Directory in which to create the page"))
                    .arg(Arg::with_name("TITLE")
                         .required(true)
                         .index(2)
                         .help("The page title"))).get_matches();

    let source_path: &Path = matches.value_of("SOURCE").unwrap_or(".").as_ref();
    match matches.subcommand() {
        ("render", Some(matches)) => {
            let dest_path: &Path = matches.value_of("DEST").unwrap().as_ref();
            if fs::metadata(&dest_path).is_ok() {
                if matches.is_present("FORCE") {
                    match fs::remove_dir_all(dest_path) {
                        Ok(_) => (),
                        Err(e) => bail!("Failed to remove '{}': {}", dest_path.display(), e),
                    }
                } else {
                    bail!("Target '{}' exists.", dest_path.display());
                }
            }
            let source = try_exit!(gazetta::Source::new(&source_path));
            try_exit!(MyGazetta.render(&source, &dest_path));
        }
        ("new", Some(matches)) => {
            let mut path: PathBuf = matches.value_of("WHERE").unwrap().into();
            let title = matches.value_of("TITLE").unwrap();
            path.push(slugify(&title));
            if fs::metadata(&path).is_ok() {
                bail!("Directory '{}' exists.", path.display());
            }
            if let Err(e) = fs::create_dir(&path) {
                bail!("Failed to create directory '{}': {}", path.display(), e);
            }
            path.push("index.md");
            let mut file = try_exit!(File::create(&path));
            try_exit!(writeln!(file, "---"));
            try_exit!(writeln!(file, "title: {}", &title));
            try_exit!(writeln!(file, "date: {}", Date::today().format("%Y-%m-%d")));
            try_exit!(writeln!(file, "---"));
            println!("Created page: {}", path.display());
            if matches.is_present("EDIT") {
                path.pop();
                match Command::new(env::var_os("EDITOR").as_ref().map(|p|&**p).unwrap_or("vim".as_ref()))
                    .arg("index.md")
                    .current_dir(path)
                    .status()
                {
                    Ok(status) => match status.code() {
                        Some(code) => process::exit(code),
                        None => bail!("Editor was killed."),
                    },
                    Err(e) => bail!("Failed to spawn editor: {}", e),
                }
            }
        },
        _ => {
            bail!("{}", matches.usage());
        }
    }
}

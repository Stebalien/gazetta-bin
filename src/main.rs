#[macro_use]
extern crate horrorshow;
extern crate gazetta;

#[macro_use]
extern crate lazy_static;

use std::{fs, io, process};
use std::io::Write;
use std::env;
use std::fmt::Display;
use std::path::PathBuf;

use gazetta::prelude::*;

mod link;
mod person;
mod meta;
mod renderer;
use renderer::MyGazetta;

macro_rules! try_exit {
    ($e:expr) => {
        match $e {
            Ok(v) => v,
            Err(e) => bail(e),
        }
    }
}

#[cold]
fn bail<M: Display>(msg: M) -> ! {
    let stderr = io::stderr();
    let mut stderr = stderr.lock();
    let _ = writeln!(stderr, "{}", msg);
    process::exit(1)
}

fn main() {
    let mut args = env::args_os();
    let _ = args.next().expect("missing arg 0? WTF!"); // Had better have args[0]
    let from: PathBuf = try_exit!(args.next().ok_or("missing source path")).into();
    let to: PathBuf = try_exit!(args.next().ok_or("missing target path")).into();
    if fs::metadata(&to).is_ok() {
        bail("target exists");
    }
    let source = try_exit!(gazetta::Source::new(&from));
    try_exit!(MyGazetta.render(&source, &to));
}

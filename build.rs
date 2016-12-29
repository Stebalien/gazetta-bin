extern crate gazetta;

use std::env;

fn main() {
    gazetta::cli::gen_completions("gazetta", &env::var("OUT_DIR").unwrap());
}

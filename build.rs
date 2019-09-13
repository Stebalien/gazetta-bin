use std::env;

use gazetta;

fn main() {
    gazetta::cli::gen_completions("gazetta", &env::var("OUT_DIR").unwrap());
}

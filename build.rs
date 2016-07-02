extern crate gazetta;

fn main() {
    gazetta::cli::gen_completions("gazetta", env!("OUT_DIR"));
}

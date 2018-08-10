#[macro_use]
extern crate clap;

mod ui;

use ui::cli;

fn main() {
    cli::parse();
}

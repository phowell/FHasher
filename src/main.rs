#[macro_use]
extern crate clap;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

mod hasher;
mod logfile;
mod ui;

use ui::cli;

fn main() {
    cli::parse();
}

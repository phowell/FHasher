#[macro_use]
extern crate clap;

extern crate walkdir;

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

extern crate md5;

mod hasher;
mod logfile;
mod ui;

use ui::cli;

fn main() {
    cli::parse();
}

#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;

mod ui;

use ui::cli;

fn main() {
    cli::parse();
}

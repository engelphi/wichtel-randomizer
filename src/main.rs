use clap::Parser;
use std::error::Error;
use std::result::Result;

fn main() -> Result<(), Box<dyn Error>> {
    let app = wichtel_randomizer::App::new(wichtel_randomizer::Args::parse());
    app.randomize()
}

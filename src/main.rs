use std::error::Error;
use std::result::Result;

fn main() -> Result<(), Box<dyn Error>> {
    let json_file = std::env::args().nth(1).expect("no input file given");
    return wichtel_randomizer::randomize(json_file);
}

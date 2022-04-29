use clap::Parser;
use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::result::Result;

/// Commandline arguments
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The input file containing the names of the persons to assign as wichtels
    #[clap(short, long)]
    input_file: String,

    /// The optional output file if not set result will be printed to stdout
    #[clap(short, long)]
    output_file: Option<String>,
}

/// model for reading the input json into
#[derive(Debug, Deserialize)]
struct Persons {
    persons: Vec<String>,
}

/// The cli application
pub struct App {
    args: Args,
}

impl App {
    /// Construct the app with the cli arguments
    pub fn new(args: Args) -> Self {
        return Self { args };
    }

    // Load persons from json file given by path
    fn read_users_from_file<P: AsRef<Path>>(&self, path: P) -> Result<Persons, Box<dyn Error>> {
        let persons = serde_json::from_reader(BufReader::new(File::open(path)?))?;
        return Ok(persons);
    }

    // Writes results to given file
    fn write_results<P: AsRef<Path>>(
        &self,
        path: P,
        map: &HashMap<String, String>,
    ) -> Result<(), Box<dyn Error>> {
        serde_json::to_writer_pretty(&File::create(path)?, &map)?;
        return Ok(());
    }

    // Writes result to stdout
    fn print_results(&self, map: &HashMap<String, String>) -> Result<(), Box<dyn Error>> {
        println!(
            "Result: {}",
            serde_json::to_string_pretty(&map).expect("Failed to serialize response")
        );
        return Ok(());
    }

    /// Entrypoint for the application logic
    /// Loads the input data from the file given via cli, calcuates the wichtels and writes
    /// the result either to stdout or to the optional file give via cli
    pub fn randomize(&self) -> Result<(), Box<dyn Error>> {
        let path = Path::new(&self.args.input_file);
        let persons = self.read_users_from_file(path).expect("no persons in file");
        let map: HashMap<String, String> =
            calculate_wichtels(&persons).expect("Unable to calculate wichtels");

        return match &self.args.output_file {
            Some(f) => self.write_results(Path::new(&f), &map),
            None => self.print_results(&map),
        };
    }
}

/**
 * Calculate wichtels for a given list of persons
 * The results is a HashMap that maps a persons from the list to another person
 * from the list.
 */
fn calculate_wichtels(persons: &Persons) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut choices = persons.persons.clone();
    let mut rng = thread_rng();
    let mut map: HashMap<String, String> = HashMap::new();
    for person in persons.persons.iter() {
        let available_choices = choices
            .iter()
            .filter(|name| **name != *person)
            .collect::<Vec<&String>>();

        let c = available_choices.choose(&mut rng);
        let choice = match c {
            None => continue,
            Some(v) => v,
        };

        map.insert(
            person.clone().as_str().to_string(),
            choice.clone().as_str().to_string(),
        );

        let index = choices
            .iter()
            .position(|name| *name == **choice)
            .expect("choice not found");

        choices.remove(index);
    }
    return Ok(map);
}

#[cfg(test)]
mod tests {
    use super::*;

    // helper for creating input data for tests
    impl Persons {
        fn new(persons: Vec<String>) -> Self {
            return Self { persons };
        }
    }

    #[test]
    fn test_calculate_wichtels() {
        let persons = Persons::new(vec![String::from("NameA"), String::from("NameB")]);
        let wichtels = calculate_wichtels(&persons).expect("");
        let expected: HashMap<String, String> = HashMap::from([
            (String::from("NameA"), String::from("NameB")),
            (String::from("NameB"), String::from("NameA")),
        ]);
        assert_eq!(expected, wichtels);
    }
}

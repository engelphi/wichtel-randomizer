use rand::seq::SliceRandom;
use rand::thread_rng;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::result::Result;

#[derive(Debug, Deserialize)]
struct Persons {
    persons: Vec<String>,
}

fn read_users_from_file<P: AsRef<Path>>(path: P) -> Result<Persons, Box<dyn Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let persons = serde_json::from_reader(reader)?;

    return Ok(persons);
}

pub fn randomize(filename: String) -> Result<(), Box<dyn Error>> {
    let path = Path::new(&filename);
    let persons = read_users_from_file(path).expect("no persons in file");

    let mut choices = persons.persons.clone();
    let mut rng = thread_rng();
    let mut map: HashMap<String, String> = HashMap::new();
    for person in persons.persons.iter() {
        let available_choices = choices
            .iter()
            .filter(|name| **name != *person)
            .collect::<Vec<&String>>();
        let choice = available_choices.choose(&mut rng).expect("No choices left");
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
    println!(
        "Result: {}",
        serde_json::to_string(&map).expect("Failed to serialize response")
    );
    return Ok(());
}

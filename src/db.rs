use serde::{Serialize, Deserialize};
use serde_yaml;
use std::fs::File;
use std::collections::HashMap;

const DB_PATH: &str = "./data/db.yaml";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    acronym: String,
    descriptions: Vec<String>,
}

impl Entry {
    pub fn new(acronym: String, description: String) -> Entry {
        Entry{
            acronym: acronym,
            descriptions: vec![description],
        }
    }
    pub fn push_descriptions(&mut self, description: String) {
        self.descriptions.push(description)
    }
}


// pub fn db_test() {
//     let foo:Entry = Entry {
//         acronym: String::from("TLA"),
//         description: String::from("Three Letter Acronym"),
//     };

//     let bar:Entry = Entry {
//         acronym: String::from("CIA"),
//         description: String::from("Central Intelligence Agency"),
//     };

//     let entries = HashMap::from([(foo.acronym.clone(), foo), (bar.acronym.clone(), bar)]);

//     write_entries(entries);

//     let entries_read: HashMap<String, Entry> = read_entries();
//     let read_foo: &Entry = entries_read.get("TLA").unwrap();
//     let read_bar = entries_read.get("CIA").unwrap();
//     println!("{read_foo:#?}");
//     println!("{read_bar:#?}");

// }


pub fn write_entries(entries: HashMap<String, Entry>) {
    serde_yaml::to_writer(File::create(DB_PATH).unwrap(), &entries).unwrap();
}


pub fn read_entries() -> HashMap<String,Entry> {
    match File::open(DB_PATH) {
        Ok(f) => serde_yaml::from_reader(f).unwrap(),
        Err(e) => {
            match e.kind() {
                std::io::ErrorKind::NotFound => HashMap::new(),
                _ => panic!("{}",e),
            }
        }
    }
}


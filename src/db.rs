use serde::{Deserialize, Serialize};
use serde_yaml;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use tempfile::Builder;

const DB_PATH: &str = "./data/db.yaml";

fn get_db_path() -> PathBuf {
    return PathBuf::from(DB_PATH);
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entry {
    acronym: String,
    descriptions: Vec<String>,
}

impl Entry {
    pub fn new(acronym: String, description: String) -> Entry {
        Entry {
            acronym: acronym,
            descriptions: vec![description],
        }
    }
    pub fn push_descriptions(&mut self, description: String) {
        self.descriptions.push(description)
    }
}

#[test]
pub fn db_test() {
    let foo = Entry::new(String::from("TLA"), String::from("Three Letter Acronym"));
    let bar = Entry::new(
        String::from("CIA"),
        String::from("Central Intelligence Agency"),
    );

    let mut entries: HashMap<String, Entry> = HashMap::new();
    entries.insert(foo.acronym.clone(), foo.clone());
    entries.insert(bar.acronym.clone(), bar.clone());

    let builder = Builder::new();
    // Generate a temporary file path without creating the file
    let temp_path = builder
        .tempdir()
        .expect("Failed to create temporary directory")
        .into_path()
        .join("tempfile");

    assert!(!temp_path.exists());

    write_entries_with_path(entries, &temp_path);

    let entries_read: HashMap<String, Entry> = read_entries_with_path(&temp_path);

    let read_foo = entries_read.get("TLA").expect("TLA entry not found");
    assert_eq!(read_foo.acronym, foo.acronym);
    assert_eq!(read_foo.descriptions, foo.descriptions);

    let read_bar = entries_read.get("CIA").expect("CIA entry not found");
    assert_eq!(read_bar.acronym, bar.acronym);
    assert_eq!(read_bar.descriptions, bar.descriptions);
}

pub fn write_entries(entries: HashMap<String, Entry>) {
    write_entries_with_path(entries, &get_db_path())
}

pub fn write_entries_with_path(entries: HashMap<String, Entry>, filepath: &PathBuf) {
    serde_yaml::to_writer(File::create(filepath).unwrap(), &entries).unwrap();
}

pub fn read_entries() -> HashMap<String, Entry> {
    return read_entries_with_path(&get_db_path());
}

pub fn read_entries_with_path(filepath: &PathBuf) -> HashMap<String, Entry> {
    match File::open(filepath) {
        Ok(f) => serde_yaml::from_reader(f).unwrap(),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => HashMap::new(),
            _ => panic!("{}", e),
        },
    }
}

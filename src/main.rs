// pub mod db;
mod cli;
mod db;

fn main() {
    println!("Hello, world!");

    let args = cli::read_args();
    println!("main {:#?}", args);
    match args.command {
        cli::Command::Add => add(args),
        cli::Command::Delete => todo!(),
    }
}


/// Adds a new entry or updates an existing one in the database.
fn add(args: cli::Args) {
    println!("Adding stuff {:#?}", args);
    let mut entries = db::read_entries();

    if let Some(entry) = entries.get_mut(&args.acronym) {
        // Update existing entry
        entry.push_descriptions(args.description);
    } else {
        // Create a new entry
        let new_entry = db::Entry::new(args.acronym.clone(), args.description);
        entries.insert(args.acronym, new_entry);
    }

    db::write_entries(entries);
}
 

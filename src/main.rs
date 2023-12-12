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


fn add(args: cli::Args) {
    println!("adding stuff {:#?}", args);
    let mut entries = db::read_entries();
    let new_entry = db::Entry::new(args.acronym.clone(),args.description);
    entries.insert(args.acronym, new_entry);
    db::write_entries(entries);
} 

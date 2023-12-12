use clap::Parser;

enum  {
    
}


#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// action you wish to perform
    #[arg(short, long)]
    command: String,

    /// acronym you wish to operate on
    #[arg(short, long)]
    acronym: String,

    /// description of the acronym
    #[arg(short, long)]
    description: String,
}

pub fn read_args() {
    let args = Args::parse();
    println!("{:#?}", args);
}



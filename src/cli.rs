use clap::{arg, builder::PossibleValue, Parser, ValueEnum};

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Command {
    Add,
    Delete,
}

impl ValueEnum for Command {
    fn value_variants<'a>() -> &'a [Self] {
        &[Command::Add, Command::Delete]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(match self {
            Command::Add => PossibleValue::new("add").help("add acroynym"),
            Command::Delete => PossibleValue::new("delete").help("delete acronym"),
        })
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for Command {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("invalid variant: {s}"))
    }
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// action you wish to perform
    #[arg(short, long)]
    pub command: Command,

    /// acronym you wish to operate on
    #[arg(short, long)]
    pub acronym: String,

    /// description of the acronym
    #[arg(short, long)]
    pub description: String,
}

pub fn read_args() -> Args {
    let args = Args::parse();
    // println!("{:#?}", args);
    return args;
}

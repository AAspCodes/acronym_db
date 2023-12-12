pub mod db;
pub mod cli;
fn main() {
    println!("Hello, world!");

    cli::read_args();
}

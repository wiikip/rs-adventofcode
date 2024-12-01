use clap::Parser;
use std::{fs::File, io::Write};
use rs_adventofcode::{bootstrap_day, get_user_input};


const SOLUTION_FOLDER: &str = "solutions";
fn main() {
    let args = Cli::parse();

    println!("Args {:?}", args.day);
    bootstrap_day(args.day, args.session_id);
}

#[derive(Parser)]
struct Cli {
    #[arg(short = 'd', long = "day")]
    day: u8,
    #[arg(short = 's')]
    session_id: String
}


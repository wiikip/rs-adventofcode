use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::fs::write;
use std::io;
use std::io::Read;

use handlebars::Handlebars;


pub mod solutions;
mod utils;

pub fn get_user_input(year: u16, day: u16, session_id: String) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    let resp = client.get(format!("https://adventofcode.com/{}/day/{}/input", year, day))
        .header(reqwest::header::COOKIE, format!("session={}", session_id))
        .send()?;
    resp.text()
}

pub fn bootstrap_day(year: u16, day: u16, session_id: String) -> Result<(), Box<dyn Error>> {
    append_day_mod(year, day)?;
    let input_folder = format!("src/solutions/y{}/d{:02}", year, day);
    fs::create_dir(&input_folder)?;
    let user_input = get_user_input(year, day, session_id)?;
    fs::write(format!("{input_folder}/input.txt"), user_input)?;
    template_day(year, day)?;
    Ok(())
}

fn append_day_mod(year: u16, day: u16) -> io::Result<()> {
    let file_content = fs::read_to_string(format!("src/solutions/y{}/mod.rs", year))?;
    let lines = file_content.split("\n");
    let mut lines_vec: Vec<&str> = lines.collect();
    let new_mod = format!("mod d{:02};", day);
    if !lines_vec.iter().any( |&s| s == new_mod.as_str()){
        lines_vec.push(new_mod.as_str());
    }
    lines_vec.sort();
    fs::write(format!("src/solutions/y{}/mod.rs", year), lines_vec.join("\n"))
}

pub fn load_day_input(year: u16, day: u8) -> io::Result<String> {
    println!("Reading file src/solutions/y{}/d{:02}/input.txt", year, day);
    fs::read_to_string(format!("src/solutions/y{}/d{:02}/input.txt", year, day))
}

pub fn template_day(year: u16, day: u16) -> Result<(), Box<dyn Error>>{
    let mut handlebars = Handlebars::new();

    handlebars.register_templates_directory(".hbs", "src/templates")?;
    let mut data = HashMap::new();
    data.insert("day", day);
    data.insert("year", year);

    let content = handlebars.render("day.rs", &data)?;
    fs::write(format!("src/solutions/y{}/d{:02}.rs", year, day), content)?;
    Ok(())
}
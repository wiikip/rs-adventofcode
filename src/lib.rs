use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs;
use std::fs::write;
use std::io;
use std::io::Read;

use handlebars::Handlebars;


mod solutions;

pub fn get_user_input(day: u8, session_id: String) -> Result<String, reqwest::Error> {
    let client = reqwest::blocking::Client::new();

    let resp = client.get(format!("https://adventofcode.com/2024/day/{}/input", day))
        .header(reqwest::header::COOKIE, format!("session={}", session_id))
        .send()?;
    resp.text()
}

pub fn bootstrap_day(day: u8, session_id: String) -> Result<(), Box<dyn Error>> {
    append_day_mod(day)?;
    let input_folder = format!("src/solutions/d{:02}", day);
    fs::create_dir(&input_folder)?;
    let user_input = get_user_input(day, session_id)?;
    fs::write(format!("{input_folder}/input.txt"), user_input)?;
    template_day(day)?;
    Ok(())
}

fn create_input_folder(day: u8) -> io::Result<()> {
    fs::create_dir(format!("src/solutions/d{:02}", day))
}

fn append_day_mod(day: u8) -> io::Result<()> {
    let file_content = fs::read_to_string("src/solutions.rs")?;
    let lines = file_content.split("\n");
    let mut lines_vec: Vec<&str> = lines.collect();
    let new_mod = format!("mod d{:02};", day);
    if !lines_vec.iter().any( |&s| s == new_mod.as_str()){
        lines_vec.push(new_mod.as_str());
    }
    lines_vec.sort();
    fs::write("src/solutions.rs", lines_vec.join("\n"))
}

pub fn load_day_input(day: u8) -> io::Result<String> {
    println!("Reading file src/solutions/d{:02}/input.txt", day);
    fs::read_to_string(format!("src/solutions/d{:02}/input.txt", day))
}

pub fn template_day(day: u8) -> Result<(), Box<dyn Error>>{
    let mut handlebars = Handlebars::new();

    handlebars.register_templates_directory(".hbs", "src/templates")?;
    let mut data = HashMap::new();
    data.insert("day", day);

    let content = handlebars.render("day.rs", &data)?;
    fs::write(format!("src/solutions/d{:02}.rs", day), content)?;
    Ok(())
}
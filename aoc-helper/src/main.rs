#![feature(exit_status_error)]
#![feature(path_try_exists)]

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs;
use std::path::Path;
use std::process::Command;

use chrono::{Datelike, Local};
use reqwest::blocking::Client;
use reqwest::header::COOKIE;
use serde::{Deserialize, Serialize};

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;

    let work_dir = std::env::current_dir().unwrap();
    let session_cookie = std::env::var("AOC_SESSION_COOKIE").expect("AOC_SESSION_COOKIE must be set");
    let cargo_toml = fs::read_to_string("Cargo.toml")?;
    let mut config: Config = toml::from_str(&cargo_toml)?;
    let days_to_init = get_days_to_init();

    for day in days_to_init {
        let crate_name = format!("day-{}", TwoDigits(day));

        if should_create_challenge_dir(&crate_name)? {
            create_challenge_dir(&crate_name, &work_dir, &mut config)?;
        }

        let input_file = format!("{}/input.txt", &crate_name);

        if should_fetch_input(&input_file)? {
            fetch_input(day, &session_cookie, &input_file)?;
        }

        println!("Initialized challenge {}", crate_name);
    }

    fs::write("Cargo.toml", toml::to_string_pretty(&config)?)?;

    Ok(())
}

fn get_days_to_init() -> Vec<u32> {
    let mut days: Vec<u32> = std::env::args()
        .filter_map(|arg| arg.parse::<u32>().ok())
        .collect();

    if days.is_empty() {
        days.push(Local::today().day());
    }

    days
}

fn should_create_challenge_dir(challenge_dir: &str) -> std::io::Result<bool> {
    Ok(!std::fs::try_exists(challenge_dir)?)
}

fn create_challenge_dir(challenge_dir: &str, work_dir: impl AsRef<Path>, config: &mut Config) -> Result<(), Box<dyn Error>> {
    let output = Command::new("cargo").current_dir(&work_dir).arg("new").arg(challenge_dir).output()?;
    output.status.exit_ok()?;
    config.workspace.members.push(challenge_dir.to_string());

    Ok(())
}

fn should_fetch_input(input_file: &str) -> std::io::Result<bool> {
    Ok(!std::fs::try_exists(input_file)?)
}

fn fetch_input(day: u32, session_cookie: &str, input_file: &str) -> Result<(), Box<dyn Error>> {
    let input_url = format!("https://adventofcode.com/2021/day/{}/input", day);
    let input = Client::new().get(input_url).header(COOKIE, format!("session={}", session_cookie)).send()?;
    fs::write(input_file, input.bytes()?)?;

    Ok(())
}

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    workspace: Workspace,
}

#[derive(Debug, Deserialize, Serialize)]
struct Workspace {
    members: Vec<String>,
}

struct TwoDigits(u32);

impl Display for TwoDigits {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let inner = self.0;

        if inner < 10 {
            write!(f, "0{}", inner)
        } else {
            write!(f, "{}", inner)
        }
    }
}

#![feature(exit_status_error)]
#![feature(path_try_exists)]

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::process::Command;
use chrono::{Datelike, Local};
use serde::{Deserialize, Serialize};
use std::fs;
use reqwest::blocking::Client;
use reqwest::header::COOKIE;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv::dotenv()?;

    let day = Local::today().day();
    let work_dir = std::env::current_dir().unwrap();
    let crate_name = format!("day-{}", TwoDigits(day));
    let session_cookie = std::env::var("AOC_SESSION_COOKIE").expect("AOC_SESSION_COOKIE must be set");

    if std::fs::try_exists(&crate_name)? {
        println!("Today's challenge is already initialized, nothing to do");
        return Ok(());
    }

    let output = Command::new("cargo").current_dir(work_dir).arg("new").arg(&crate_name).output()?;
    output.status.exit_ok()?;

    let cargo_toml = fs::read_to_string("Cargo.toml")?;
    let mut config: Config = toml::from_str(&cargo_toml)?;
    config.workspace.members.push(crate_name.clone());
    fs::write("Cargo.toml", toml::to_string_pretty(&config)?)?;

    let input_url = format!("https://adventofcode.com/2021/day/{}/input", day);
    let input = Client::new().get(input_url).header(COOKIE, format!("session={}", session_cookie)).send()?;
    let input_file = format!("{}/input.txt", &crate_name);
    fs::write(input_file, input.bytes()?)?;

    println!("Initialized challenge {}", crate_name);

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

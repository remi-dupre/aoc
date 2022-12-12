//! Tools used to fetch input contents from adventofcode.com.

use std::fs::{self, create_dir_all, read_to_string, File};
use std::io::{self, Write};
use std::io::{stdin, stdout};
use std::path::{Path, PathBuf};
use std::time::Instant;

use attohttpc::header::{COOKIE, USER_AGENT};
use regex::Regex;

use crate::params::InputChoice;
use crate::utils::Line;
use crate::{input, Day, Error, Year};

const BASE_URL: &str = "https://adventofcode.com";
const USER_AGENT_VALUE: &str = "github.com/remi-dupre/aoc by remi@dupre.io";

fn input_path(year: u16, day: u8) -> PathBuf {
    format!("input/{year}/day{day}.txt").into()
}

fn output_path(year: u16, day: u8, part: u8) -> PathBuf {
    format!("output/{year}/day{day}-{part}.txt").into()
}

pub fn get_input_data(day: Day, year: Year, input: &InputChoice) -> String {
    match input {
        InputChoice::Download => input::get_input(year, day).expect("could not fetch input"),
        InputChoice::File(file) => fs::read_to_string(file).expect("failed to read from stdin"),
        InputChoice::Stdin => {
            let input = io::stdin().lock();
            io::read_to_string(input).expect("failed to read specified file")
        }
    }
}

fn token_path() -> PathBuf {
    dirs::config_dir()
        .map(|mut cfg| {
            cfg.push("aoc/token.txt");
            cfg
        })
        .unwrap_or_else(|| ".token".into())
}

fn session_cookie() -> Result<String, Error> {
    let token = get_conn_token()?;
    Ok(format!("session={token}"))
}

fn get_input(year: u16, day: u8) -> Result<String, Error> {
    let url = format!("{}/{}/day/{}/input", BASE_URL, year, day);

    let fetch_from_web = move || {
        let start = Instant::now();

        let resp = attohttpc::get(&url)
            .header(COOKIE, session_cookie()?)
            .header(USER_AGENT, USER_AGENT_VALUE)
            .send()?;

        let elapsed = start.elapsed();
        let mut result = resp.text()?;

        Line::new("download input file")
            .with_duration(elapsed)
            .println();

        if result.ends_with('\n') {
            result.pop();
        }

        Ok(result)
    };

    get_from_path_or_else(&input_path(year, day), fetch_from_web)
}

pub fn get_expected(year: u16, day: u8, part: u8) -> Result<Option<String>, Error> {
    let pattern =
        Regex::new(r"Your puzzle answer was <code>(.*)</code>\.").expect("could no build pattern");

    let url = format!("https://adventofcode.com/{year}/day/{day}");

    let fetch_from_web = move || {
        let start = Instant::now();

        let resp = attohttpc::get(&url)
            .header(COOKIE, session_cookie()?)
            .header(USER_AGENT, USER_AGENT_VALUE)
            .send()?;

        let elapsed = start.elapsed();
        let body = resp.text()?;
        let line = Line::new("get expected").with_duration(elapsed);

        let Some(found) = pattern.captures_iter(&body).nth(usize::from(part) - 1) else {
            line.println();
            return Ok(None);
        };

        let expected = found.get(1).expect("no capture in pattern").as_str();
        line.with_output(expected).println();
        Ok(Some(expected.to_string()))
    };

    try_get_from_path_or_else(&output_path(year, day, part), fetch_from_web)
}

fn get_conn_token() -> Result<String, Error> {
    get_from_path_or_else(&token_path(), || {
        let mut stdout = stdout();
        write!(&mut stdout, "Write your connection token: ")?;
        stdout.flush()?;

        let mut output = String::new();
        stdin().read_line(&mut output)?;
        Ok(output.trim().to_string())
    })
}

fn get_from_path_or_else(
    path: &Path,
    fallback: impl FnOnce() -> Result<String, Error>,
) -> Result<String, Error> {
    let from_path = read_to_string(path);

    if let Ok(res) = from_path {
        Ok(res)
    } else {
        let res = fallback()?;

        create_dir_all(path.parent().expect("no parent directory"))
            .and_then(|_| File::create(path))
            .and_then(|mut file| file.write_all(res.as_bytes()))
            .unwrap_or_else(|err| eprintln!("could not write {}: {}", path.display(), err));

        Ok(res)
    }
}

fn try_get_from_path_or_else(
    path: &Path,
    fallback: impl FnOnce() -> Result<Option<String>, Error>,
) -> Result<Option<String>, Error> {
    let from_path = read_to_string(path);

    if let Ok(res) = from_path {
        Ok(Some(res))
    } else {
        let res = fallback()?;

        if let Some(res) = &res {
            create_dir_all(path.parent().expect("no parent directory"))
                .and_then(|_| File::create(path))
                .and_then(|mut file| file.write_all(res.as_bytes()))
                .unwrap_or_else(|err| eprintln!("could not write {}: {}", path.display(), err));
        }

        Ok(res)
    }
}

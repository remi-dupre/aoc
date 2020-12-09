//! Tools used to fetch input contents from adventofcode.com.

use std::error::Error;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;
use std::io::{stdin, stdout};
use std::path::{Path, PathBuf};
use std::time::Instant;

use crate::utils::Line;

const BASE_URL: &str = "https://adventofcode.com";

fn input_path(year: u16, day: u8) -> PathBuf {
    format!("input/{}/day{}.txt", year, day).into()
}

fn token_path() -> PathBuf {
    dirs::config_dir()
        .map(|mut cfg| {
            cfg.push("aoc/token.txt");
            cfg
        })
        .unwrap_or_else(|| ".token".into())
}

pub fn get_input(year: u16, day: u8) -> Result<String, Box<dyn Error>> {
    let mut result = get_from_path_or_else(&input_path(year, day), || {
        let start = Instant::now();
        let url = format!("{}/{}/day/{}/input", BASE_URL, year, day);
        let session_cookie = format!("session={}", get_conn_token()?);
        let resp = attohttpc::get(&url)
            .header(attohttpc::header::COOKIE, session_cookie)
            .send()?;
        let elapsed = start.elapsed();

        println!(
            "  - {}",
            Line::new("downloaded input file").with_duration(elapsed)
        );

        resp.text()
    })?;

    if result.ends_with('\n') {
        result.pop();
    }

    Ok(result)
}

fn get_conn_token() -> Result<String, std::io::Error> {
    get_from_path_or_else(&token_path(), || {
        let mut stdout = stdout();
        write!(&mut stdout, "Write your connection token: ")?;
        stdout.flush()?;

        let mut output = String::new();
        stdin().read_line(&mut output)?;
        Ok(output.trim().to_string())
    })
}

fn get_from_path_or_else<E: Error>(
    path: &Path,
    fallback: impl FnOnce() -> Result<String, E>,
) -> Result<String, E> {
    let from_path = read_to_string(path);

    if let Ok(res) = from_path {
        Ok(res.trim().to_string())
    } else {
        let res = fallback()?;
        create_dir_all(path.parent().expect("no parent directory"))
            .and_then(|_| File::create(path))
            .and_then(|mut file| file.write_all(res.as_bytes()))
            .unwrap_or_else(|err| eprintln!("could not write {}: {}", path.display(), err));
        Ok(res)
    }
}

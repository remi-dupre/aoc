use std::cmp::min;
use std::error::Error;
use std::fs::{create_dir_all, read_to_string, File};
use std::io::Write;
use std::io::{stdin, stdout};
use std::iter;
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub use clap::Clap;
pub use colored;
use colored::*;

const DISPLAY_WIDTH: usize = 40;
const BASE_URL: &str = "https://adventofcode.com";
const INPUT_DIR: &str = "input";
const CONN_TOKEN_FILE: &str = ".token";

#[derive(Debug, Clap)]
#[clap(
    name = "Advent of Code",
    about = concat!("Main page of the event: https://adventofcode.com/")
)]
pub struct Opt {
    /// Read input from stdin instead of downloading it
    #[clap(short, long)]
    pub stdin: bool,

    /// Days to execute. By default all implemented days will run.
    #[clap(short, long)]
    pub days: Vec<String>,
}

pub fn print_with_duration(line: &str, output: Option<&str>, duration: Duration) {
    let duration = format!("({:.2?})", duration);
    print!("  - {} {}", line, duration.dimmed());

    if let Some(output) = output {
        let width = "  - ".len() + line.chars().count() + 1 + duration.chars().count();
        let dots = DISPLAY_WIDTH - min(DISPLAY_WIDTH - 5, width) - 2;
        let dots: String = iter::repeat('.').take(dots).collect();
        print!(" {}", dots.dimmed());

        if output.contains('\n') {
            println!();

            for line in output.trim_matches('\n').lines() {
                println!("    {}", line.bold());
            }
        } else {
            println!(" {}", output.bold());
        }
    } else {
        println!()
    }
}

fn input_path(year: u16, day: u8) -> String {
    format!("{}/{}/day{}.txt", INPUT_DIR, year, day)
}

fn get_from_path_or_else<E: Error>(
    path: &str,
    fallback: impl FnOnce() -> Result<String, E>,
) -> Result<String, E> {
    let from_path = read_to_string(path);

    if let Ok(res) = from_path {
        Ok(res.trim().to_string())
    } else {
        let res = fallback()?;
        create_dir_all(PathBuf::from(path).parent().unwrap())
            .and_then(|_| File::create(path))
            .and_then(|mut file| file.write_all(res.as_bytes()))
            .unwrap_or_else(|err| eprintln!("could not write {}: {}", path, err));
        Ok(res)
    }
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

        print_with_duration("downloaded input file", None, elapsed);
        resp.text()
    })?;

    if result.ends_with('\n') {
        result.pop();
    }

    Ok(result)
}

pub fn get_conn_token() -> Result<String, std::io::Error> {
    get_from_path_or_else(CONN_TOKEN_FILE, || {
        let mut stdout = stdout();
        write!(&mut stdout, "Write your connection token: ")?;
        stdout.flush()?;

        let mut output = String::new();
        stdin().read_line(&mut output)?;

        let mut file = File::create(CONN_TOKEN_FILE)?;
        file.write_all(output.as_bytes())?;
        Ok(output.trim().to_string())
    })
}

#[macro_export]
macro_rules! main {
    (
        year $year: expr;
        $( $day: ident $( : $generator: ident )? => $( $solution: ident ),+ );+
        $( ; )?
    ) => {
        use std::time::Instant;
        use std::io::Read;

        use $crate::Clap;

        const YEAR: u16 = $year;
        const DAYS: &[&str] = &[$(stringify!($day)),*];

        fn main() {
            let mut opt = $crate::Opt::parse();

            if opt.days.is_empty() {
                opt.days = DAYS.iter().map(|s| s[3..].to_string()).collect();
            }

            for (i, day) in opt.days.iter().enumerate() {$({
                let module_name = format!("day{}", day);
                let day = day.parse().expect("days must be integers");

                if !DAYS.contains(&module_name.as_str()) {
                    eprintln!("Module `{}` was not registered", module_name);
                }

                if stringify!($day) == module_name {
                    if i != 0 { println!() }
                    println!("Day {}", day);

                    let data = {
                        if opt.stdin {
                            let mut data = String::new();
                            std::io::stdin().read_to_string(&mut data)
                                .expect("failed to read from stdin");
                            data
                        } else {
                            $crate::get_input(YEAR, day).expect("could not fetch input")
                        }
                    };

                    let input = data.as_str();

                    $(
                        let start = Instant::now();
                        let input = $day::$generator(&data);
                        let elapsed = start.elapsed();
                        $crate::print_with_duration("generator", None, elapsed);
                    )?

                    $({
                        let start = Instant::now();
                        let response = $day::$solution(&input);
                        let elapsed = start.elapsed();

                        $crate::print_with_duration(
                            stringify!($solution),
                            Some(&format!("{}", response)),
                            elapsed,
                        );
                    })+
                }
            })+}
        }
    };
}

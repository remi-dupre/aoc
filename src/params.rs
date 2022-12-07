use std::path::PathBuf;

use clap::{Arg, ArgAction, Command, ValueHint};

use crate::Day;

pub enum InputChoice {
    Download,
    Stdin,
    File(PathBuf),
}

pub enum DayChoice {
    All,
    Latest,
    Select(Vec<Day>),
}

impl DayChoice {
    pub fn has_multiple_choices(&self) -> bool {
        match self {
            DayChoice::All => true,
            DayChoice::Latest => false,
            DayChoice::Select(days) => days.len() > 1,
        }
    }
}

pub struct Params {
    pub input: InputChoice,
    pub days: DayChoice,
    pub bench: bool,
}

pub fn get_params(year: u16) -> Params {
    let args = Command::new(format!("Advent of Code {year}"))
        .about(format!(
            "Main page of the event: https://adventofcode.com/{year}/"
        ))
        .arg(
            Arg::new("stdin")
                .short('i')
                .long("stdin")
                .action(ArgAction::SetTrue)
                .conflicts_with("file")
                .help("Read input from stdin instead of downloading it"),
        )
        .arg(
            Arg::new("file")
                .short('f')
                .long("file")
                .conflicts_with("stdin")
                .value_hint(ValueHint::FilePath)
                .help("Read input from file instead of downloading it"),
        )
        .arg(
            Arg::new("days")
                .short('d')
                .long("day")
                .value_name("day num")
                .help("Days to execute. By default only latest days will run"),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .action(ArgAction::SetTrue)
                .conflicts_with("days")
                .help("Run all days"),
        )
        .arg(
            Arg::new("bench")
                .short('b')
                .long("bench")
                .action(ArgAction::SetTrue)
                .help("Run criterion benchmarks"),
        )
        .get_matches();

    let input = match (args.get_flag("stdin"), args.get_one::<PathBuf>("file")) {
        (false, None) => InputChoice::Download,
        (true, None) => InputChoice::Stdin,
        (false, Some(path)) => InputChoice::File(path.clone()),
        (true, Some(_)) => panic!("You can't specify both stdin and file inputs"),
    };

    let days = match (args.get_one::<Vec<Day>>("days"), args.get_flag("all")) {
        (None, false) => DayChoice::Latest,
        (None, true) => DayChoice::All,
        (Some(days), false) => DayChoice::Select(days.clone()),
        (Some(_), true) => panic!("You can't specify days with the --all option"),
    };

    let bench = args.get_flag("bench");
    Params { input, days, bench }
}

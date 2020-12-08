use std::path::PathBuf;

use clap::{Clap, Subcommand};

#[derive(Debug, Clap)]
#[clap(
    name = "Advent of Code",
    about = concat!("Main page of the event: https://adventofcode.com/")
)]
pub struct Opt {
    /// Read input from stdin instead of downloading it
    #[clap(short = 'i', long, conflicts_with = "file")]
    pub stdin: bool,

    /// Read input from file instead of downloading it
    #[clap(short, long, conflicts_with = "stdin")]
    pub file: Option<PathBuf>,

    /// Days to execute. By default all implemented days will run.
    #[clap(short, long = "day", name = "day num")]
    pub days: Vec<String>,

    // TODO: better handling of bench CLI
    /// Run criterion benchmarks
    #[clap(short, long)]
    pub bench: bool,

    #[clap(subcommand)]
    pub cmd: Option<SubOpt>,
}

#[derive(Debug, Subcommand)]
pub enum SubOpt {
    Bench(BenchOpt),
}

#[derive(Debug, Clap)]
pub struct BenchOpt {}

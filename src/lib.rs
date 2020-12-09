pub mod input;
pub mod parse;
pub mod utils;

use std::path::PathBuf;

// Reexport some crates for the generated main
pub use clap;
pub use colored;

#[cfg(feature = "bench")]
pub use criterion;

use clap::Clap;

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
    #[clap(name = "day num", short, long = "day")]
    pub days: Vec<String>,

    // TODO: better handling of bench CLI
    /// Run criterion benchmarks
    #[clap(short, long)]
    pub bench: bool,
}

impl Opt {
    pub fn day_enabled(&self, day: &str) -> bool {
        day.starts_with("day") && self.days.iter().any(|d| d == &day[3..])
    }
}

#[macro_export]
macro_rules! base_main {
    ( year $year: expr; $( $tail: tt )* ) => {
        use std::fs::read_to_string;
        use std::io::Read;
        use std::time::Instant;

        use $crate::clap::Clap;
        use $crate::{bench_day, extract_day, parse, run_day};

        const YEAR: u16 = $year;

        fn main() {
            let mut opt = $crate::Opt::parse();

            if opt.bench {
                bench();
            } else {
                if opt.days.is_empty() {
                    opt.days = parse!(extract_day {}; $( $tail )*)
                        .iter()
                        .map(|s| s[3..].to_string())
                        .collect();
                } else {
                    let days = parse! { extract_day {}; $( $tail )* };

                    let ignored_days: Vec<_> = opt.days
                        .iter()
                        .filter(|day| !days.contains(&format!("day{}", day).as_str()))
                        .map(String::as_str)
                        .collect();

                    if !ignored_days.is_empty() {
                        eprintln!(r"/!\ Ignoring unimplemented days: {}", ignored_days.join(", "));
                    }

                    opt.days = opt.days
                        .into_iter()
                        .filter(|day| days.contains(&format!("day{}", day).as_str()))
                        .collect();
                }

                if opt.days.len() > 1 && (opt.stdin || opt.file.is_some()) {
                    eprintln!(r"/!\ You are using a personalized output over several days which can");
                    eprintln!(r"    be missleading. If you only intend to run solutions for a");
                    eprintln!(r"    specific day, you can specify it by using the `-d DAY_NUM` flag.");
                }

                for (i, day) in opt.days.iter().enumerate() {
                    parse! {
                        run_day { i, format!("day{}", day), YEAR, opt };
                        $( $tail )*
                    };
                }
            }
        }
    }
}

#[cfg(feature = "bench")]
#[macro_export]
macro_rules! main {
    ( year $year: expr; $( $tail: tt )* ) => {
        $crate::base_main! { year $year; $( $tail )* }

        use $crate::criterion::Criterion;

        fn bench() {
            let mut criterion = Criterion::default().configure_from_args();

            parse! {
                bench_day { &mut criterion, YEAR };
                $( $tail )*
            };

            criterion.final_summary();
        }
    }
}

#[cfg(not(feature = "bench"))]
#[macro_export]
macro_rules! main {
    ( year $year: expr; $( $tail: tt )* ) => {
        $crate::base_main! { year $year; $( $tail )* }

        fn bench() {
            println!("Benchmarks not available, please enable `bench` feature for cargo-main.");
        }
    }
}

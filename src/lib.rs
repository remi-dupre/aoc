pub mod input;
pub mod parse;
pub mod utils;

// Reexport some crates for the generated main
pub use clap;
pub use colored;

#[cfg(feature = "bench")]
pub use criterion;

use clap::{App, Arg, ValueHint};

pub fn args(year: u16) -> App<'static> {
    App::new(format!("Advent of Code {}", year))
        .author("Rémi D. <remi@dupre.io>")
        .about("Main page of the event: https://adventofcode.com/")
        .arg(
            Arg::new("stdin")
                .short('i')
                .long("stdin")
                .conflicts_with("file")
                .help("Read input from stdin instead of downloading it"),
        )
        .arg(
            Arg::new("file")
                .takes_value(true)
                .short('f')
                .long("file")
                .conflicts_with("stdin")
                .value_hint(ValueHint::FilePath)
                .help("Read input from file instead of downloading it"),
        )
        .arg(
            Arg::new("days")
                .takes_value(true)
                .short('d')
                .long("day")
                .value_name("day num")
                .help("Days to execute. By default all implemented days will run"),
        )
        .arg(
            Arg::new("bench")
                .short('b')
                .long("bench")
                .help("Run criterion benchmarks"),
        )
        .arg(
            Arg::new("all")
                .short('a')
                .long("all")
                .conflicts_with("days")
                .help("Run all days"),
        )
}

#[macro_export]
macro_rules! base_main {
    ( year $year: expr; $( $tail: tt )* ) => {
        use std::fs::read_to_string;
        use std::io::Read;
        use std::time::Instant;

        use $crate::{bench_day, extract_day, parse, run_day};

        const YEAR: u16 = $year;

        fn main() {
            let mut opt = $crate::args(YEAR).get_matches();

            let days: Vec<_> = {
                if let Some(opt_days) = opt.values_of("days") {
                    let opt_days: Vec<_> = opt_days.collect();
                    let days = parse! { extract_day {}; $( $tail )* };

                    let ignored_days: Vec<_> = opt_days
                        .iter()
                        .filter(|day| !days.contains(&format!("day{}", day).as_str()))
                        .copied()
                        .collect();

                    if !ignored_days.is_empty() {
                        eprintln!(r"/!\ Ignoring unimplemented days: {}", ignored_days.join(", "));
                    }

                    opt_days
                        .into_iter()
                        .filter(|day| days.contains(&format!("day{}", day).as_str()))
                        .collect()
                } else if opt.is_present("all") {
                    parse!(extract_day {}; $( $tail )*)
                        .iter()
                        .map(|s| &s[3..])
                        .collect()
                } else {
                    // Get most recent day, assuming the days are sorted
                    vec![parse!(extract_day {}; $( $tail )*)
                        .iter()
                        .map(|s| &s[3..])
                        .last()
                        .expect("No day implemenations found")]
                }
            };

            if opt.is_present("bench") {
                bench(days);
            } else {
                if days.len() > 1 && (opt.is_present("stdin") || opt.is_present("file")) {
                    eprintln!(r"/!\ You are using a personalized output over several days which can");
                    eprintln!(r"    be missleading. If you only intend to run solutions for a");
                    eprintln!(r"    specific day, you can specify it by using the `-d DAY_NUM` flag.");
                }

                for (i, day) in days.iter().enumerate() {
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

        fn bench(days: Vec<&str>) {
            let mut criterion = Criterion::default().with_output_color(true);

            for day in days.into_iter() {
                parse! {
                    bench_day { &mut criterion, format!("day{}", day), YEAR };
                    $( $tail )*
                };
            }

            criterion.final_summary();
        }
    }
}

#[cfg(not(feature = "bench"))]
#[macro_export]
macro_rules! main {
    ( year $year: expr; $( $tail: tt )* ) => {
        $crate::base_main! { year $year; $( $tail )* }

        fn bench(days: Vec<&str>) {
            println!("Benchmarks not available, please enable `bench` feature for cargo-main.");
        }
    }
}

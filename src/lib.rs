pub mod input;

use std::cmp::min;
use std::iter;
use std::time::Duration;

pub use clap::Clap;
pub use colored;
use colored::*;

const DISPLAY_WIDTH: usize = 40;

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

            for (i, day) in opt.days.iter().enumerate() {
                let module_name = format!("day{}", day);
                let day = day.parse().expect("days must be integers");

                if !DAYS.contains(&module_name.as_str()) {
                    eprintln!(
                        "Module `{}` was not registered, available are: {}",
                        module_name,
                        DAYS.join(", "),
                    );
                }

                $(
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
                                $crate::input::get_input(YEAR, day).expect("could not fetch input")
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
                )+
            }
        }
    };
}

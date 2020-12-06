pub mod input;

use std::cmp::min;
use std::iter;
use std::path::PathBuf;
use std::time::Duration;

// Reexport some crates for the generated main
pub use clap;
pub use criterion;

use clap::Clap;
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

#[macro_export]
macro_rules! main {
    (
        year $year: expr;
        $( $day: ident $( : $generator: ident )? => $( $solution: ident ),+ );+
        $( ; )?
    ) => {
        use std::fs::read_to_string;
        use std::io::Read;
        use std::time::Instant;

        use $crate::clap::Clap;

        const YEAR: u16 = $year;
        const DAYS: &[&str] = &[$(stringify!($day)),*];

        fn main() {
            let mut opt = $crate::Opt::parse();

            if opt.bench {
                bench::run_benchs();
            }

            if opt.days.is_empty() {
                opt.days = DAYS.iter().map(|s| s[3..].to_string()).collect();
            } else {
                let ignored_days: Vec<_> = opt.days
                    .iter()
                    .filter(|day| !DAYS.contains(&format!("day{}", day).as_str()))
                    .map(String::as_str)
                    .collect();

                if !ignored_days.is_empty() {
                    eprintln!(r"/!\ Ignoring unimplemented days: {}", ignored_days.join(", "));
                }

                opt.days = opt.days
                    .into_iter()
                    .filter(|day| DAYS.contains(&format!("day{}", day).as_str()))
                    .collect();
            }

            if opt.days.len() > 1 && (opt.stdin || opt.file.is_some()) {
                eprintln!(r"/!\ You are using a personalized output over several days which can");
                eprintln!(r"    be missleading. If you only intend to run solutions for a");
                eprintln!(r"    specific day, you can specify it by using the `-d DAY_NUM` flag.");
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
                            } else if let Some(path) = opt.file.as_ref() {
                                read_to_string(path)
                                    .expect("failed to read specified file")
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


        mod bench {
            use $crate::criterion::*;

            pub fn run_benchs() {
                main();
            }

            $(
                fn $day(c: &mut Criterion) {
                    let mut group = c.benchmark_group(stringify!($day));
                    let day = stringify!($day)[3..].parse().expect("dayX expected for module");

                    let data = $crate::input::get_input(crate::YEAR, day)
                        .expect("could not fetch input");

                    let input = data.as_str();
                    $( let input = crate::$day::$generator(&data); )?


                    $(
                        group.bench_function(
                            stringify!($solution),
                            |b| b.iter(|| crate::$day::$solution(&input)),
                        );
                    )+

                    group.finish();
                }
            )+

            criterion_group!(benches, $($day),+);
            criterion_main!(benches);
        }
    };
}

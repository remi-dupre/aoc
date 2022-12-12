pub mod input;
pub mod params;
pub mod step;
pub mod utils;

use std::borrow::Borrow;
use std::fmt::Display;
use std::time::Instant;

use colored::Colorize;

use crate::input::{get_expected, get_input_data};
use crate::params::{get_params, DayChoice, InputChoice, Params};
use crate::step::Step;
use crate::utils::{leak, Line, Status};

pub type Error = Box<dyn std::error::Error>;

type Day = u8;
type Year = u16;

pub struct Solution<I: 'static> {
    ident: &'static str,
    implem: Box<dyn Step<&'static I, String>>,
}

pub struct DaySolutions<I: 'static> {
    year: Year,
    day: Day,
    generator: Box<dyn Step<&'static mut String, I>>,
    solutions: Vec<Solution<I>>,
}

impl<I> DaySolutions<I> {
    pub fn new<G>(year: Year, day: Day, generator: G) -> Self
    where
        G: Step<&'static mut String, I> + 'static,
    {
        let generator = Box::new(generator);

        Self {
            year,
            day,
            generator,
            solutions: Vec::new(),
        }
    }

    pub fn with_solution<V, F, O>(mut self, ident: &'static str, implem: F) -> Self
    where
        V: ?Sized + 'static,
        I: Borrow<V> + 'static,
        F: Step<&'static V, O> + 'static,
        O: Display + 'static,
    {
        let implem =
            Box::new(move |input: &'static I| implem.run(input.borrow()).map(|x| x.to_string()));

        self.solutions.push(Solution { ident, implem });
        self
    }
}

pub trait DayTrait {
    fn day(&self) -> Day;
    fn run(&self, params: &Params);

    #[cfg(feature = "bench")]
    fn bench(&self, params: &Params, criterion: &mut criterion::Criterion);
}

impl<I> DayTrait for DaySolutions<I> {
    fn day(&self) -> Day {
        self.day
    }

    fn run(&self, params: &Params) {
        println!("Day {}:", self.day);
        let data = leak(get_input_data(self.day, self.year, &params.input));
        let extract_part = regex::Regex::new(r"part(\d+)").unwrap();

        let input = {
            let start = Instant::now();
            let res = self.generator.run(data);
            let line = Line::new("generator").with_duration(start.elapsed());

            match res {
                Ok(x) => {
                    line.println();
                    leak(x)
                }
                Err(err) => {
                    line.with_output(err.red()).println();
                    return;
                }
            }
        };

        for solution in &self.solutions {
            let start = Instant::now();
            let res = solution.implem.run(input);
            let line = Line::new(solution.ident).with_duration(start.elapsed());

            let get_expected = || {
                let part: u8 = extract_part
                    .captures(solution.ident)?
                    .get(1)?
                    .as_str()
                    .parse()
                    .ok()?;

                // TODO: display errors
                get_expected(self.year, self.day(), part)
                    .map_err(|err| eprintln!("{err}"))
                    .ok()
                    .flatten()
            };

            let line = line.with_status(match get_expected() {
                None => Status::Warn,
                x if x.as_ref() == res.as_ref().ok() => Status::Ok,
                _ => Status::Err,
            });

            match res {
                Ok(x) => line.with_output(x.normal()).println(),
                Err(err) => line.with_output(err.red()).println(),
            }
        }
    }

    #[cfg(feature = "bench")]
    fn bench(&self, params: &Params, criterion: &mut criterion::Criterion) {
        let mut group = criterion.benchmark_group(format!("day{}", self.day));
        let data = leak(get_input_data(self.day, self.year, &params.input));

        let input = {
            let res = self.generator.run(data);

            match res {
                Ok(x) => x,
                Err(err) => {
                    eprintln!(
                        r"/!\ Skipping day {} because generator failed: {err}",
                        self.day,
                    );

                    return;
                }
            }
        };

        let input = leak(input);

        for solution in &self.solutions {
            group.bench_function(solution.ident, |b| b.iter(|| solution.implem.run(input)));
            // group.bench_with_input(solution.ident, input, |b, input| {
            //     b.iter(move || solution.implem.run(input))
            // });
        }

        group.finish();
    }
}

pub fn parse_day_ident(ident: &str) -> Result<Day, String> {
    let day = ident
        .strip_prefix("day")
        .ok_or_else(|| "day modules should be in the form dayXX".to_string())?;

    day.parse()
        .map_err(|err| format!("invalid day {day}: {err}"))
}

pub fn run_main(year: Year, days: &[Box<dyn DayTrait>]) {
    let params = get_params(year);

    if !matches!(params.input, InputChoice::Download) && params.days.has_multiple_choices() {
        eprintln!(r"/!\ You are using a personalized output over several days which can");
        eprintln!(r"    be missleading. If you only intend to run solutions for a");
        eprintln!(r"    specific day, you can specify it by using the `-d DAY_NUM` flag.");
        eprintln!();
    }

    let days: Vec<_> = match params.days {
        DayChoice::All => days.iter().collect(),
        DayChoice::Latest => days.last().into_iter().collect(),
        DayChoice::Select(ref selected) => selected
            .iter()
            .filter_map(|selected_day| days.iter().find(|day| day.day() == *selected_day))
            .collect(),
    };

    if params.bench {
        #[cfg(feature = "bench")]
        {
            let mut criterion = criterion::Criterion::default()
                .with_output_color(true)
                .warm_up_time(std::time::Duration::from_millis(200))
                .measurement_time(std::time::Duration::from_millis(1000));

            for day in days {
                day.bench(&params, &mut criterion);
            }

            criterion.final_summary();
        }
        #[cfg(not(feature = "bench"))]
        {
            eprintln!(r"/!\ You are using option --bench but the 'bench' feature is disabled,");
            eprintln!(r"     please update dependancy to aoc-main in your Cargo.toml.");
        }
    } else {
        for day in days {
            day.run(&params);
        }
    }
}

#[macro_export]
macro_rules! with_fallback {
    ( $true: expr, $false: expr ) => {
        $true
    };
    ( , $false: expr ) => {
        $false
    };
}

#[macro_export]
macro_rules! main {
    (
        year $year: expr;
        $(
            $day: ident
            $( : $generator: ident $( ? $([$($_gen:tt)* $opt_gen:tt])? )? )?
            => $( $part: ident $( ? $([$($_imp:tt)* $opt_imp:tt])? )? ),+ ; )*
    ) => {
        use $crate::{DaySolutions, DayTrait, parse_day_ident, run_main};
        use $crate::step::{GeneratorInput, InfaillibleStep, Step};

        const YEAR: u16 = $year;

        fn main() {
            let days: &[Box<dyn DayTrait>] = &[
                $(Box::new(
                    DaySolutions::new(
                        YEAR,
                        parse_day_ident(&stringify!($day)).expect("failed to parse day"),
                        $crate::with_fallback!(
                            $(|mut input| {
                                $crate::with_fallback!(
                                    $( $($opt_gen)? $day::$generator )?,
                                    InfaillibleStep($day::$generator)
                                ).run(GeneratorInput::take_buffer(input))
                            })?,
                            InfaillibleStep(|x: &'static mut String| std::mem::take(x))
                        )
                    )

                    $(
                        .with_solution(
                            stringify!($part),
                            $crate::with_fallback!(
                                $( $($opt_imp)? $day::$part )?,
                                InfaillibleStep($day::$part)
                            ),
                        )
                    )*
                )),+
            ];

            run_main(YEAR, &days);
        }
    }
}

pub mod input;
pub mod params;
pub mod step;
pub mod utils;

use std::borrow::Borrow;
use std::fmt::Display;
use std::time::Instant;

// Reexport some crates for the generated main
pub use clap;
pub use colored;

use colored::Colorize;

#[cfg(feature = "bench")]
pub use criterion;

use crate::input::get_input_data;
use crate::utils::Line;

use self::params::{get_params, DayChoice, InputChoice, Params};
use self::step::Step;

pub type Error = Box<dyn std::error::Error>;

type Day = u8;
type Year = u16;

pub struct Solution<I> {
    ident: &'static str,
    implem: Box<dyn for<'a> Step<&'a I, String>>,
}

pub struct DaySolutions<I> {
    year: Year,
    day: Day,
    generator: Box<dyn Step<String, I>>,
    solutions: Vec<Solution<I>>,
}

impl<I> DaySolutions<I> {
    pub fn new<G>(year: Year, day: Day, generator: G) -> Self
    where
        G: for<'a> Step<String, I> + 'static,
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
        V: ?Sized,
        I: Borrow<V>,
        F: for<'a> Step<&'a V, O> + 'static,
        O: Display,
    {
        let implem =
            Box::new(move |input: &'_ I| implem.run(input.borrow()).map(|x| x.to_string()));

        self.solutions.push(Solution { ident, implem });
        self
    }
}

pub trait DayTrait {
    fn day(&self) -> Day;
    fn run(&self, params: &Params);
}

impl<I> DayTrait for DaySolutions<I> {
    fn day(&self) -> Day {
        self.day
    }

    fn run(&self, params: &Params) {
        println!("Day {}:", self.day);
        let data = get_input_data(self.day, self.year, &params.input);

        let input = {
            let start = Instant::now();
            let res = self.generator.run(data);
            let line = Line::new("generator").with_duration(start.elapsed());

            match res {
                Ok(x) => {
                    line.println();
                    x
                }
                Err(err) => {
                    line.with_state(err.red()).println();
                    return;
                }
            }
        };

        for solution in &self.solutions {
            let start = Instant::now();
            let res = solution.implem.run(&input);
            let line = Line::new(solution.ident).with_duration(start.elapsed());

            match res {
                Ok(x) => line.with_state(x.normal()).println(),
                Err(err) => line.with_state(err.red()).println(),
            }
        }
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

    for day in days {
        day.run(&params);
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
                                ).run(GeneratorInput::take_buffer(&mut input))
                            })?,
                            InfaillibleStep(|x: String| x)
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

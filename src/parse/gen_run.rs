//! Generate code to run solutions.

#[macro_export]
macro_rules! run_day {
    (
        { $i: expr, $curr_day: expr, $year: expr, $opt: expr },
        { day $day: ident { $gen: tt { $( $sol: tt )* } } }
    ) => {{
        if stringify!($day) == $curr_day {
            if $i != 0 { println!() }
            let day = $curr_day[3..].parse().expect("days must be integers");
            println!("Day {}", day);

            let data = {
                if $opt.is_present("stdin") {
                    let mut data = String::new();
                    std::io::stdin().read_to_string(&mut data)
                        .expect("failed to read from stdin");
                    data
                } else if let Some(path) = $opt.value_of("file") {
                    read_to_string(path)
                        .expect("failed to read specified file")
                } else {
                    $crate::input::get_input($year, day).expect("could not fetch input")
                }
            };

            if let Some(input) = $crate::run_gen!($day, &data, $gen) {
                $( $crate::run_sol!($day, &input, $sol); )+
            } else {
                $( $crate::skip_sol!($sol); )+
            }
        }
    }}
}

#[macro_export]
macro_rules! run_gen {
    // No generator is needed: default behavior is to just pass input &str
    ( $day: ident, $data: expr, { gen_default } ) => {{
        Some($data)
    }};

    // Run generator
    ( $day: ident, $data: expr, { gen $generator: ident } ) => {{
        use $crate::utils::Line;

        let start = Instant::now();
        let input = $day::$generator($data);
        let elapsed = start.elapsed();
        println!("  - {}", Line::new("generator").with_duration(elapsed));
        Some(input)
    }};

    // Run fallible generator
    ( $day: ident, $data: expr, { gen_fallible $generator: ident } ) => {{
        use $crate::colored::*;
        use $crate::utils::{Line, TryUnwrap};

        let start = Instant::now();
        let result = $day::$generator($data);
        let elapsed = start.elapsed();

        match result.try_unwrap() {
            Ok(input) => {
                println!("  - {}", Line::new("generator").with_duration(elapsed));
                Some(input)
            }
            Err(msg) => {
                println!(
                    "  - {}",
                    Line::new("generator")
                        .with_duration(elapsed)
                        .with_state(msg.red())
                );
                None
            }
        }
    }};
}

#[macro_export]
macro_rules! run_sol {
    // Run solution
    ( $day: ident, $input: expr, { sol $solution: ident } ) => {{
        use $crate::colored::*;
        use $crate::utils::Line;

        let start = Instant::now();
        let response = $day::$solution($input);
        let elapsed = start.elapsed();

        println!(
            "  - {}",
            Line::new(stringify!($solution))
                .with_duration(elapsed)
                .with_state(format!("{}", response).normal())
        );
    }};

    // Run fallible solution
    ( $day: ident, $input: expr, { sol_fallible $solution: ident } ) => {{
        use $crate::colored::*;
        use $crate::utils::{Line, TryUnwrap};

        let start = Instant::now();
        let response = $day::$solution($input);
        let elapsed = start.elapsed();
        let line = Line::new(stringify!($solution)).with_duration(elapsed);

        println!(
            "  - {}",
            match response.try_unwrap() {
                Ok(response) => {
                    line.with_state(format!("{}", response).normal())
                }
                Err(msg) => {
                    line.with_state(msg.red())
                }
            }
        );
    }};
}

#[macro_export]
macro_rules! skip_sol {
    ({ $kind: tt $solution: ident }) => {{
        use $crate::colored::*;
        use $crate::utils::Line;

        println!(
            "  - {}",
            Line::new(stringify!($solution)).with_state("skipped".bright_black())
        );
    }};
}

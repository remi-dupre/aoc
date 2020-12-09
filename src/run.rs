#[macro_export]
macro_rules! run_day {
    (
        { $i: expr, $curr_day: expr, $year: expr, $opt: expr },
        { day $day: ident { $gen: tt { $( $sol: tt )* } } }
    ) => {{
        use $crate::colored::*;

        if stringify!($day) == $curr_day {
            if $i != 0 { println!() }
            let day = $curr_day[3..].parse().expect("days must be integers");
            println!("Day {}", day);

            let data = {
                if $opt.stdin {
                    let mut data = String::new();
                    std::io::stdin().read_to_string(&mut data)
                        .expect("failed to read from stdin");
                    data
                } else if let Some(path) = $opt.file.as_ref() {
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
    // No generator is needed: default begavior is to just pass input &str
    ( $day: ident, $data: expr, { gen_default } ) => {{
        Some($data)
    }};

    // Run generator
    ( $day: ident, $data: expr, { gen $generator: ident } ) => {{
        let start = Instant::now();
        let input = $day::$generator($data);
        let elapsed = start.elapsed();
        $crate::print_with_duration("generator", None, Some(elapsed));
        Some(input)
    }};

    // Run fallible generator
    ( $day: ident, $data: expr, { gen_fallible $generator: ident } ) => {{
        use $crate::colored::*;
        use $crate::try_unwrap::TryUnwrap;

        let start = Instant::now();
        let result = $day::$generator($data);
        let elapsed = start.elapsed();

        match result.try_unwrap() {
            Ok(input) => {
                $crate::print_with_duration("generator", None, Some(elapsed));
                Some(input)
            }
            Err(msg) => {
                $crate::print_with_duration("generator", Some(msg.red()), Some(elapsed));
                None
            }
        }
    }};
}

#[macro_export]
macro_rules! run_sol {
    // Run solution
    ( $day: ident, $input: expr, { sol $solution: ident } ) => {{
        let start = Instant::now();
        let response = $day::$solution($input);
        let elapsed = start.elapsed();

        $crate::print_with_duration(
            stringify!($solution),
            Some(format!("{}", response).normal()),
            Some(elapsed),
        );
    }};

    // Run fallible solution
    ( $day: ident, $input: expr, { sol_fallible $solution: ident } ) => {{
        use $crate::colored::*;
        use $crate::try_unwrap::TryUnwrap;

        let start = Instant::now();
        let response = $day::$solution($input);
        let elapsed = start.elapsed();

        match response.try_unwrap() {
            Ok(response) => {
                $crate::print_with_duration(
                    stringify!($solution),
                    Some(format!("{}", response).normal()),
                    Some(elapsed),
                );
            }
            Err(msg) => {
                $crate::print_with_duration(stringify!($solution), Some(msg.red()), Some(elapsed));
            }
        }
    }};
}

#[macro_export]
macro_rules! skip_sol {
    ({ $kind: tt $solution: ident }) => {{
        $crate::print_with_duration(stringify!($solution), Some("skipped".dimmed()), None);
    }};
}

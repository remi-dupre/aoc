#[macro_export]
macro_rules! run_day {
    (
        { $i: expr, $curr_day: expr, $year: expr, $opt: expr },
        { day $day: ident { $gen: tt { $( { sol $solution: ident } )* } } }
    ) => {{
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

            let input = $crate::run_gen!($day, &data, $gen);

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
    }}
}

#[macro_export]
macro_rules! run_gen {
    // No generator is needed: default begavior is to just pass input &str
    ( $day: ident, $data: expr, { gen_default } ) => {{
        $data
    }};

    // Run generator
    ( $day: ident, $data: expr, { gen $generator: ident } ) => {{
        let start = Instant::now();
        let input = $day::$generator($data);
        let elapsed = start.elapsed();
        $crate::print_with_duration("generator", None, elapsed);
        input
    }};
}

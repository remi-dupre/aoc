//! Generate code for criterion benchmarks.

#[macro_export]
macro_rules! bench_day {
    (
        { $criterion: expr, $curr_day: expr, $year: expr },
        { day $day: ident { $gen: tt { $( $sol: tt )* } } }
    ) => {{
        if stringify!($day) == $curr_day {
            let day = $curr_day[3..].parse().expect("days must be integers");
            let data = $crate::input::get_input($year, day).expect("could not fetch input");
            let input = $crate::bench_gen!($day, &data, $gen);

            let mut group = $criterion.benchmark_group(stringify!($day));
            $( $crate::bench_sol!(&mut group, $day, &input, $sol); )+
            group.finish();
        }
    }}
}

// This is just a silent version of run_gen with more agressive exceptions.
#[macro_export]
macro_rules! bench_gen {
    ( $day: ident, $data: expr, { gen_default } ) => {{
        $data
    }};
    ( $day: ident, $data: expr, { gen $generator: ident } ) => {{
        $day::$generator($data)
    }};
    ( $day: ident, $data: expr, { gen_fallible $generator: ident } ) => {{
        use std::fmt::*;
        $day::$generator($data).expect("failed to parse input")
    }};
}

#[macro_export]
macro_rules! bench_sol {
    ( $group: expr, $day: ident, $input: expr, { $kind: tt $solution: ident } ) => {{
        $group.bench_function(stringify!($solution), |b| {
            b.iter(|| $day::$solution($input))
        });
    }};
}

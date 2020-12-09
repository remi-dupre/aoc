mod day1 {
    pub fn generator(input: &str) -> Vec<u64> {
        input
            .lines()
            .map(|line| {
                line.parse()
                    .unwrap_or_else(|err| panic!("invalid number `{}`: `{}`", line, err))
            })
            .collect()
    }

    pub fn part_1(input: &[u64]) -> u64 {
        input.iter().map(|&mass| mass / 3 - 2).sum()
    }

    pub fn part_2(input: &[u64]) -> u64 {
        fn total_needed_mass(obj: u64) -> u64 {
            if obj < 9 {
                0
            } else {
                let obj_mass = obj / 3 - 2;
                obj_mass + total_needed_mass(obj_mass)
            }
        }

        input.iter().copied().map(total_needed_mass).sum()
    }
}

macro_rules! debug_tt {
    ( $( $tt:tt )* ) => {{
        println!("{}", stringify!($($tt)*));
    }}
}

macro_rules! run {
    ({}, { day $day: ident { gen $generator: ident { $( { sol $solution: ident } )* } } }) => {{
        let input = $day::$generator("42");

        $({
            println!("{}", $day::$solution(&input));
        })*
    }}
}

// fn main() {
//     aoc_main::parse! {
//         debug_tt {};
//         day1 : generator => part_1, part_2;
//         day2 : generator => part_1_array, part_1;
//     };
//
//     aoc_main::parse! {
//         run {};
//         day1 : generator => part_1, part_2;
//     };
//
//     let test = aoc_main::parse! {
//         extract_day {};
//         day1 : generator => part_1, part_2;
//         day2 : generator => part_1_array, part_1;
//     };
//     dbg!(test);
//
// }

use aoc::extract_day;

aoc::main! {
    year 2019;
    day1 : generator => part_1, part_2;
}

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

mod day2 {
    pub fn part_1(input: &str) -> usize {
        input.len()
    }
}

mod day3 {
    pub fn generator(_: &str) -> Option<&str> {
        None
    }

    pub fn part_1(input: &str) -> usize {
        input.len()
    }
}

mod day4 {
    pub fn generator(_: &str) -> Result<i64, impl std::fmt::Display> {
        "five".parse()
    }

    pub fn part_1(input: &i64) -> i64 {
        *input
    }
}

aoc::main! {
    year 2019;
    day1 : generator  => part_1, part_2;
    day2              => part_1;
    day3 : generator? => part_1;
    day4 : generator? => part_1;
}

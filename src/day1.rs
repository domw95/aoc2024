use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

type Input = String;

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

#[aoc(day1, part1)]
fn solver_part1(input: &Input) -> u32 {
    dbg!(input);
    0
}

#[aoc(day1, part2)]
fn solver_part2(input: &Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::solver_part1;

    #[test]
    fn sample1() {
        assert_eq!(solver_part1(&input_generator("input")), 0)
    }
}

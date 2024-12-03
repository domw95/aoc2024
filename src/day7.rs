use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

type Input = String;

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

#[aoc(day7, part1)]
fn solver_part1(input: &Input) -> u32 {
    dbg!(input);
    0
}

#[aoc(day7, part2)]
fn solver_part2(input: &Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 0)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 0)
    }
}

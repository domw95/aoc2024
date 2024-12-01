use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

type Input = (Vec<i64>, Vec<i64>);

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            //
            let (a, b) = l.split_once("   ").unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .unzip()
}

#[aoc(day1, part1)]
fn solver_part1(input: &Input) -> i64 {
    let mut input = input.clone();
    input.0.sort();
    input.1.sort();
    input
        .0
        .iter()
        .zip(input.1)
        .map(|(a, b)| {
            //
            (a - b).abs()
        })
        .sum()
}

#[aoc(day1, part2)]
fn solver_part2(input: &Input) -> usize {
    input
        .0
        .iter()
        .map(|a| input.1.iter().filter(|b| *b == a).count() * *a as usize)
        .sum()
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

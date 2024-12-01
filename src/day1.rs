use std::collections::HashMap;

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

#[aoc(day1, part2, FILTER)]
fn solver_part2_filter(input: &Input) -> usize {
    input
        .0
        .iter()
        .map(|a| input.1.iter().filter(|b| *b == a).count() * *a as usize)
        .sum()
}

#[aoc(day1, part2, HASH)]
fn solver_part2_hash(input: &Input) -> i64 {
    let mut hash = HashMap::new();
    for v in &input.1 {
        match hash.get(v) {
            Some(x) => hash.insert(*v, x + 1),
            None => hash.insert(*v, 1),
        };
    }

    input
        .0
        .iter()
        .map(|a| {
            //
            match hash.get(a) {
                Some(x) => a * x,
                None => 0,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::solver_part1;

    #[test]
    fn sample1() {
        assert_eq!(
            solver_part1(&input_generator(
                "3   4
4   3
2   5
1   3
3   9
3   3"
            )),
            11
        )
    }
}

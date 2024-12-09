use std::collections::HashMap;
use std::thread;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

type Input = String;

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

fn parse(input: &str) -> (Vec<i64>, Vec<i64>) {
    input
        .lines()
        .map(|l| {
            //
            let (a, b) = l.split_once("   ").unwrap();
            (a.parse::<i64>().unwrap(), b.parse::<i64>().unwrap())
        })
        .unzip()
}

fn parse_i32(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|l| {
            //
            let (a, b) = l.split_once("   ").unwrap();
            (a.parse::<i32>().unwrap(), b.parse::<i32>().unwrap())
        })
        .unzip()
}

fn fast_parse_i32(input: &str) -> (Vec<i32>, Vec<i32>) {
    input
        .lines()
        .map(|l| {
            //
            let (a, b) = l.split_once("   ").unwrap();
            (a.parse::<i32>().unwrap_or(0), b.parse::<i32>().unwrap_or(0))
        })
        .unzip()
}

#[aoc(day1, part1)]
fn solver_part1(input: &Input) -> i64 {
    let mut input = parse(input);
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

#[aoc(day1, part1, UNSTABLE)]
fn solver_part1_unstable(input: &Input) -> i64 {
    let mut input = parse(input);
    input.0.sort_unstable();
    input.1.sort_unstable();
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

#[aoc(day1, part1, UNSTABLE_I32)]
fn solver_part1_unstable_i32(input: &Input) -> i32 {
    let mut input = parse_i32(input);
    input.0.sort_unstable();
    input.1.sort_unstable();
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

#[aoc(day1, part1, UNSTABLE_I32_PARALLEL)]
fn solver_part1_unstable_i32_parallel(input: &Input) -> i32 {
    let (mut a, mut b) = parse_i32(input);
    let handle = thread::spawn(|| {
        a.sort_unstable();
        a
    });
    b.sort_unstable();
    let a = handle.join().unwrap();
    a.iter()
        .zip(b)
        .map(|(a, b)| {
            //
            (a - b).abs()
        })
        .sum()
}

#[aoc(day1, part1, UNSTABLE_I32_FAST)]
fn solver_part1_unstable_i32_fast(input: &Input) -> i32 {
    let mut input = fast_parse_i32(input);
    input.0.sort_unstable();
    input.1.sort_unstable();
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

fn parse_map(input: &str) -> (Vec<i32>, HashMap<i32, i32>) {
    let mut vec = Vec::new();
    let mut map = HashMap::new();
    for l in input.lines() {
        let (a, b) = l.split_once("   ").unwrap();
        let (a, b) = (a.parse::<i32>().unwrap_or(0), b.parse::<i32>().unwrap_or(0));
        match map.get_mut(&b) {
            Some(v) => *v += 1,
            None => {
                map.insert(b, 1);
            }
        };
        vec.push(a);
    }
    (vec, map)
}

#[aoc(day1, part2)]
fn solver_part2(input: &Input) -> usize {
    let input = parse(input);
    input
        .0
        .iter()
        .map(|a| input.1.iter().filter(|b| *b == a).count() * *a as usize)
        .sum()
}

#[aoc(day1, part2, HASH)]
fn solver_part2_hash(input: &Input) -> i64 {
    let input = parse(input);
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

#[aoc(day1, part2, HASH_INLINE)]
fn solver_part2_hash_inline(input: &Input) -> i32 {
    let input = parse_map(input);

    input
        .0
        .iter()
        .map(|a| {
            //
            match input.1.get(a) {
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

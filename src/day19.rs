#![allow(clippy::comparison_chain)]
use std::collections::HashMap;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Input = String;

#[aoc_generator(day19)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

fn possible_pattern(pattern: &[u8], towels: &[&[u8]]) -> bool {
    // println!("Checking {}", String::from_utf8(pattern.to_vec()).unwrap());
    towels.iter().any(|&towel| {
        // println!("? {}", String::from_utf8(towel.to_vec()).unwrap());
        if towel.len() == pattern.len() {
            pattern == towel
        } else if towel.len() < pattern.len() {
            if towel.iter().zip(pattern.iter()).all(|(a, b)| a == b) {
                // println!("Match {}", String::from_utf8(towel.to_vec()).unwrap());
                possible_pattern(&pattern[towel.len()..], towels)
            } else {
                false
            }
        } else {
            false
        }
    })
}

fn possible_pattern_count<'a>(
    pattern: &'a [u8],
    towels: &[&[u8]],
    cache: &mut HashMap<&'a [u8], usize>,
) -> usize {
    // println!("Checking {}", String::from_utf8(pattern.to_vec()).unwrap());
    if let Some(count) = cache.get(pattern) {
        *count
    } else {
        let count = towels
            .iter()
            .map(|&towel| {
                // println!("? {}", String::from_utf8(towel.to_vec()).unwrap());
                if towel.len() == pattern.len() {
                    if pattern == towel {
                        1
                    } else {
                        0
                    }
                } else if towel.len() < pattern.len() {
                    if towel.iter().zip(pattern.iter()).all(|(a, b)| a == b) {
                        // println!("Match {}", String::from_utf8(towel.to_vec()).unwrap());
                        possible_pattern_count(&pattern[towel.len()..], towels, cache)
                    } else {
                        0
                    }
                } else {
                    0
                }
            })
            .sum();
        cache.insert(pattern, count);
        count
    }
}

#[aoc(day19, part1)]
fn solver_part1(input: &Input) -> usize {
    let mut lines = input.lines();
    let towels = lines
        .next()
        .unwrap()
        .split(',')
        .map(|str| str.trim().as_bytes())
        .collect_vec();
    lines.next();
    lines
        .map(str::as_bytes)
        .filter(|&pattern| possible_pattern(pattern, &towels))
        .count()
}

#[aoc(day19, part2)]
fn solver_part2(input: &Input) -> usize {
    let mut lines = input.lines();
    let mut cache = HashMap::new();
    let towels = lines
        .next()
        .unwrap()
        .split(',')
        .map(|str| str.trim().as_bytes())
        .collect_vec();
    lines.next();
    lines
        .map(str::as_bytes)
        .map(|pattern| possible_pattern_count(pattern, &towels, &mut cache))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 6)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 16)
    }
}

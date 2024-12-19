#![allow(clippy::comparison_chain)]
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use dashmap::DashMap;
use fxhash::FxHashMap;
use itertools::Itertools;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelBridge;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSlice;

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

fn possible_pattern_count_fx<'a>(
    pattern: &'a [u8],
    towels: &[&[u8]],
    cache: &mut FxHashMap<&'a [u8], usize>,
) -> usize {
    // println!("Checking {}", String::from_utf8(pattern.to_vec()).unwrap());
    if let Some(count) = cache.get(pattern) {
        *count
    } else {
        let count = towels
            .iter()
            .map(|&towel| {
                // println!("? {}", String::from_utf8(towel.to_vec()).unwrap());
                if pattern == towel {
                    1
                } else if towel.len() < pattern.len() {
                    if towel.iter().zip(pattern.iter()).all(|(a, b)| a == b) {
                        // println!("Match {}", String::from_utf8(towel.to_vec()).unwrap());
                        possible_pattern_count_fx(&pattern[towel.len()..], towels, cache)
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

fn possible_pattern_count_dash<'a>(
    pattern: &'a [u8],
    towels: &[&[u8]],
    cache: &DashMap<&'a [u8], usize>,
) -> usize {
    // println!("Checking {}", String::from_utf8(pattern.to_vec()).unwrap());
    if let Some(count) = cache.get(pattern) {
        return *count;
    }

    let mut count = 0;
    for &towel in towels {
        // println!("? {}", String::from_utf8(towel.to_vec()).unwrap());
        if pattern == towel {
            count += 1
        } else if towel.len() < pattern.len()
            && towel.iter().zip(pattern.iter()).all(|(a, b)| a == b)
        {
            // println!("Match {}", String::from_utf8(towel.to_vec()).unwrap());
            count += possible_pattern_count_dash(&pattern[towel.len()..], towels, cache);
        }
    }
    cache.insert(pattern, count);
    count
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

#[aoc(day19, part1, PARALLEL)]
fn solver_part1_parallel(input: &Input) -> usize {
    let mut lines = input.lines();
    let towels = lines
        .next()
        .unwrap()
        .split(',')
        .map(|str| str.trim().as_bytes())
        .collect_vec();
    lines.next();
    lines
        .par_bridge()
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

#[aoc(day19, part2, PARALLEL)]
fn solver_part2_parallel(input: &Input) -> usize {
    let mut lines = input.lines();
    let cache = HashMap::new();
    let towels = lines
        .next()
        .unwrap()
        .split(',')
        .map(|str| str.trim().as_bytes())
        .collect_vec();
    lines.next();
    lines
        .par_bridge()
        .map(str::as_bytes)
        .map(|pattern| possible_pattern_count(pattern, &towels, &mut cache.clone()))
        .sum()
}

#[aoc(day19, part2, PARALLEL_DASH)]
fn solver_part2_parallel_dash(input: &Input) -> usize {
    let mut lines = input.lines();
    let cache = DashMap::new();
    let towels = lines
        .next()
        .unwrap()
        .split(',')
        .map(|str| str.trim().as_bytes())
        .collect_vec();
    lines.next();
    lines
        .par_bridge()
        .map(str::as_bytes)
        .map(|pattern| possible_pattern_count_dash(pattern, &towels, &cache))
        .sum()
}

#[aoc(day19, part2, PARALLEL_WITH)]
fn solver_part2_parallel_with(input: &Input) -> usize {
    let mut lines = input.lines();
    let cache = HashMap::new();
    let towels = lines
        .next()
        .unwrap()
        .split(',')
        .map(|str| str.trim().as_bytes())
        .collect_vec();
    lines.next();
    lines
        .par_bridge()
        .map(str::as_bytes)
        .map_with(cache, |cache, pattern| {
            possible_pattern_count(pattern, &towels, cache)
        })
        .sum()
}

#[aoc(day19, part2, PARALLEL_CHUNKS)]
fn solver_part2_parallel_chunks(input: &Input) -> usize {
    let mut lines = input.lines();
    let cache = HashMap::new();
    let towels = lines
        .next()
        .unwrap()
        .split(',')
        .map(|str| str.trim().as_bytes())
        .collect_vec();
    lines.next();
    let lines = lines.collect_vec();
    lines
        .par_chunks(5)
        .map(|chunk| {
            chunk
                .iter()
                .map(|str| possible_pattern_count(str.as_bytes(), &towels, &mut cache.clone()))
                .sum::<usize>()
        })
        .sum()
}

#[aoc(day19, part2, PARALLEL_WITH_FX)]
fn solver_part2_parallel_with_fx(input: &Input) -> usize {
    let mut lines = input.lines();
    let cache = FxHashMap::default();
    let towels = lines
        .next()
        .unwrap()
        .split(',')
        .map(|str| str.trim().as_bytes())
        .collect_vec();
    lines.next();
    lines
        .par_bridge()
        .map(str::as_bytes)
        .map_with(cache, |cache, pattern| {
            possible_pattern_count_fx(pattern, &towels, cache)
        })
        .sum()
}

#[aoc(day19, part2, PARALLEL_MANUAL)]
fn solver_part2_parallel_manual(input: &Input) -> usize {
    let mut lines = input.lines();

    let towels = lines
        .next()
        .unwrap()
        .split(',')
        .map(|str| str.trim().as_bytes())
        .collect_vec();
    lines.next();
    let lines = Arc::new(Mutex::new(lines));
    let global_cache = Arc::new(Mutex::new(HashMap::new()));
    (0..12)
        .into_par_iter()
        .map(|_| {
            let mut count = 0;
            let lines = lines.clone();
            let mut cache = HashMap::new();
            loop {
                let mut guard = lines.lock().unwrap();
                if let Some(line) = guard.next() {
                    drop(guard);
                    // let mut cache = global_cache.lock().unwrap().clone();
                    count += possible_pattern_count(line.as_bytes(), &towels, &mut cache);
                    let mut global = global_cache.lock().unwrap();
                    global.extend(cache);
                    cache = global.clone();
                } else {
                    break;
                }
            }
            count
        })
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

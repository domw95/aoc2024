use std::collections::HashMap;
use std::time::Instant;

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use fxhash::FxHashMap;
use itertools::Itertools;
use rayon::prelude::*;

type Input = String;

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

fn recursive(v: u64, blinks: u32) -> usize {
    if blinks == 0 {
        1
    } else {
        match v {
            0 => recursive(1, blinks - 1),
            v => {
                let digits = digits(v);
                if digits % 2 == 0 {
                    recursive(v / 10u64.pow(digits / 2), blinks - 1)
                        + recursive(v % 10u64.pow(digits / 2), blinks - 1)
                } else {
                    recursive(v * 2024, blinks - 1)
                }
            }
        }
    }
}

fn recursive_cache(v: u64, blinks: u32, cache: &mut HashMap<(u64, u32), usize>) -> usize {
    if blinks == 0 {
        1
    } else if let Some(count) = cache.get(&(v, blinks)) {
        *count
    } else {
        let count = match v {
            0 => recursive_cache(1, blinks - 1, cache),
            v => {
                let digits = digits(v);
                if digits % 2 == 0 {
                    recursive_cache(v / 10u64.pow(digits / 2), blinks - 1, cache)
                        + recursive_cache(v % 10u64.pow(digits / 2), blinks - 1, cache)
                } else {
                    recursive_cache(v * 2024, blinks - 1, cache)
                }
            }
        };
        cache.insert((v, blinks), count);
        count
    }
}

fn recursive_cache_fx(v: u64, blinks: u32, cache: &mut FxHashMap<(u64, u32), usize>) -> usize {
    if blinks == 0 {
        1
    } else if let Some(count) = cache.get(&(v, blinks)) {
        *count
    } else {
        let count = match v {
            0 => recursive_cache_fx(1, blinks - 1, cache),
            v => {
                let digits = digits(v);
                if digits % 2 == 0 {
                    recursive_cache_fx(v / 10u64.pow(digits / 2), blinks - 1, cache)
                        + recursive_cache_fx(v % 10u64.pow(digits / 2), blinks - 1, cache)
                } else {
                    recursive_cache_fx(v * 2024, blinks - 1, cache)
                }
            }
        };
        cache.insert((v, blinks), count);
        count
    }
}

fn recursive_cache_fx_custom(v: u64, blinks: u32, cache: &mut [FxHashMap<u64, usize>]) -> usize {
    if blinks == 0 {
        1
    } else if let Some(count) = cache[blinks as usize - 1].get(&v) {
        *count
    } else {
        let count = match v {
            0 => recursive_cache_fx_custom(1, blinks - 1, cache),
            v => {
                let digits = digits(v);
                if digits % 2 == 0 {
                    recursive_cache_fx_custom(v / 10u64.pow(digits / 2), blinks - 1, cache)
                        + recursive_cache_fx_custom(v % 10u64.pow(digits / 2), blinks - 1, cache)
                } else {
                    recursive_cache_fx_custom(v * 2024, blinks - 1, cache)
                }
            }
        };
        cache[blinks as usize - 1].insert(v, count);
        count
    }
}

fn digits(value: u64) -> u32 {
    value.ilog10() + 1
}

fn bfs(blinks: u32, values: Vec<u64>) -> usize {
    let mut map = FxHashMap::default();
    for v in values {
        map.insert(v, 1);
    }
    for _ in 0..blinks {
        let mut new_map = FxHashMap::default();
        for (v, count) in map {
            if v == 0 {
                new_map.entry(1);
            } else {
                let digits = digits(v);
                if digits % 2 == 0 {
                    // new_map.
                }
            }
        }
        map = new_map;
    }
    map.values().sum()
}

fn split_vec(vec: &[u64]) -> Vec<u64> {
    let mut new = Vec::new();
    for v in vec {
        if *v == 0 {
            new.push(1);
        } else {
            let digits = digits(*v);
            if digits % 2 == 0 {
                new.push(v / 10u64.pow(digits / 2));
                new.push(v % 10u64.pow(digits / 2));
            } else {
                new.push(v * 2024);
            }
        }
    }
    new
}
#[aoc(day11, part1)]
fn solver_part1(input: &Input) -> usize {
    input
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .map(|v| recursive(v, 25))
        .sum()
}

#[aoc(day11, part1, BFS)]
fn solver_part1_bfs(input: &Input) -> usize {
    bfs(
        25,
        input
            .split_ascii_whitespace()
            .map(|str| str.parse::<u64>().unwrap())
            .collect(),
    )
}

#[aoc(day11, part1, Parallel)]
fn solver_part1_parallel(input: &Input) -> usize {
    input
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .par_bridge()
        .map(|v| recursive(v, 25))
        .sum()
}

#[aoc(day11, part1, FX_Custom)]
fn solver_part1_fx_custom(input: &Input) -> usize {
    let mut cache: [_; 25] = core::array::from_fn(|_| FxHashMap::default());
    input
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .map(|v| recursive_cache_fx_custom(v, 25, &mut cache))
        .sum()
}

#[aoc(day11, part1, Parallel_FX)]
fn solver_part1_parallel_fx(input: &Input) -> usize {
    let cache = FxHashMap::default();
    input
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .par_bridge()
        .map(|v| recursive_cache_fx(v, 25, &mut cache.clone()))
        .sum()
}

#[aoc(day11, part1, Parallel_FX_Custom)]
fn solver_part1_parallel_fx_custom(input: &Input) -> usize {
    let cache: [_; 25] = core::array::from_fn(|_| FxHashMap::default());
    input
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .par_bridge()
        .map(|v| recursive_cache_fx_custom(v, 25, &mut cache.clone()))
        .sum()
}

#[aoc(day11, part2)]
fn solver_part2(input: &Input) -> usize {
    let mut cache = HashMap::new();
    input
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        // .par_bridge()
        .map(|v| recursive_cache(v, 75, &mut cache))
        .sum()
}

// #[aoc(day11, part2, Parallel)]
// fn solver_part2_parallel(input: &Input) -> usize {
//     for i in 1..75 {
//         let now = Instant::now();
//         let res: usize = input
//             .split_ascii_whitespace()
//             .map(|str| str.parse::<u64>().unwrap())
//             // .par_bridge()
//             .map(|v| recursive(v, i))
//             .sum();
//         println!("{i},{res},{}", now.elapsed().as_secs_f32());
//     }
//     0
// }

#[aoc(day11, part2, Parallel)]
fn solver_part2_parallel(input: &Input) -> usize {
    let mut vec = input
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .collect_vec();
    let mut splits = 0;
    for i in 1..75 {
        let now = Instant::now();
        let res: usize = if vec.len() < 60 {
            vec = split_vec(&vec);
            splits += 1;
            vec.len()
        } else {
            vec.iter()
                .par_bridge()
                .map(|v| recursive(*v, i - splits))
                .sum()
        };
        println!("{i},{res},{}", now.elapsed().as_secs_f32());
    }
    0
}

#[aoc(day11, part2, Parallel_Cache)]
fn solver_part2_parallel_cache(input: &Input) -> usize {
    let cache = HashMap::new();
    input
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .par_bridge()
        .map(|v| recursive_cache(v, 75, &mut cache.clone()))
        .sum()
}

#[aoc(day11, part2, FX_Custom)]
fn solver_part2_fx_custom(input: &Input) -> usize {
    let mut cache: [_; 75] = core::array::from_fn(|_| FxHashMap::default());
    input
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .map(|v| recursive_cache_fx_custom(v, 75, &mut cache))
        .sum()
}

#[aoc(day11, part2, Parallel_FX)]
fn solver_part2_parallel_fx(input: &Input) -> usize {
    let cache = FxHashMap::default();
    input
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .par_bridge()
        .map(|v| recursive_cache_fx(v, 75, &mut cache.clone()))
        .sum()
}

#[aoc(day11, part2, Parallel_FX_Custom)]
fn solver_part2_parallel_fx_custom(input: &Input) -> usize {
    let cache: [_; 75] = core::array::from_fn(|_| FxHashMap::default());
    input
        .split_ascii_whitespace()
        .map(|str| str.parse::<u64>().unwrap())
        .par_bridge()
        .map(|v| recursive_cache_fx_custom(v, 75, &mut cache.clone()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "125 17";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 55312)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 0)
    }
}
// Parallel
// 25, 189547, 0.002321947 secs
// 26, 285429, 0.002869066 secs
// 27, 433672, 0.004342216 secs
// 28, 661693, 0.007310164 secs
// 29, 998273, 0.008637174 secs
// 30, 1525214, 0.012924604 secs
// 31, 2306931, 0.0200478 secs
// 32, 3508534, 0.029771648 secs
// 33, 5351940, 0.05281378 secs
// 34, 8067746, 0.07117383 secs
// 35, 12309881, 0.1099453 secs
// 36, 18705685, 0.15778589 secs
// 37, 28336072, 0.26471668 secs
// 38, 43176239, 0.39001015 secs
// 39, 65378277, 0.5465312 secs
// 40, 99491570, 0.78886616 secs
// 41, 151182056, 1.3046187 secs
// 42, 229081370, 1.8242849 secs
// 43, 348989228, 2.963923 secs
// 44, 529126588, 4.626022 secs
// 45, 803609759, 6.3659363 secs
// 46, 1222382207, 10.422769 secs
// 47, 1853535428, 15.7769 secs
// 48, 2819248327, 24.26915 secs
// 49, 4279506546, 35.72901 secs
// 50, 6497541585, 57.78131 secs
// 51, 9881561849, 86.551094 secs

// Parallel cached
// 25, 189547, 0.000772283 secs
// 26, 285429, 0.000311906 secs
// 27, 433672, 0.000394317 secs
// 28, 661693, 0.000461009 secs
// 29, 998273, 0.000525864 secs
// 30, 1525214, 0.000519661 secs
// 31, 2306931, 0.000687413 secs
// 32, 3508534, 0.000624733 secs
// 33, 5351940, 0.000665598 secs
// 34, 8067746, 0.000703387 secs
// 35, 12309881, 0.000750264 secs
// 36, 18705685, 0.000699568 secs
// 37, 28336072, 0.001015447 secs
// 38, 43176239, 0.001386031 secs
// 39, 65378277, 0.001511037 secs
// 40, 99491570, 0.001351674 secs
// 41, 151182056, 0.001422406 secs
// 42, 229081370, 0.001769998 secs
// 43, 348989228, 0.001695483 secs
// 44, 529126588, 0.002098656 secs
// 45, 803609759, 0.002106576 secs
// 46, 1222382207, 0.003076667 secs
// 47, 1853535428, 0.002418631 secs
// 48, 2819248327, 0.003302503 secs
// 49, 4279506546, 0.003635382 secs
// 50, 6497541585, 0.003038492 secs
// 51, 9881561849, 0.004090815 secs
// 52, 14989831626, 0.004060075 secs
// 53, 22783880272, 0.004086939 secs
// 54, 34611747294, 0.005778815 secs
// 55, 52535276048, 0.005478017 secs
// 56, 79865768259, 0.005158271 secs
// 57, 121234753538, 0.005038309 secs
// 58, 184178398765, 0.006073105 secs
// 59, 279837397329, 0.00754918 secs
// 60, 424797840901, 0.005915957 secs
// 61, 645605535731, 0.0090223 secs
// 62, 980372352184, 0.008289777 secs
// 63, 1488934975775, 0.010625921 secs
// 64, 2262406077367, 0.009179925 secs
// 65, 3435039839919, 0.008758131 secs
// 66, 5218963206220, 0.008905573 secs
// 67, 7926948856006, 0.009797746 secs
// 68, 12038235032256, 0.009476169 secs
// 69, 18290482441458, 0.009831162 secs
// 70, 27775066436548, 0.01531861 secs
// 71, 42192151976831, 0.01559641 secs
// 72, 64091881828909, 0.016395953 secs
// 73, 97332869199706, 0.011404402 secs
// 74, 147869130284377, 0.011573302 secs
// 75, 224577979481346, 0.012423619 secs

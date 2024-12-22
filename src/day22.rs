use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;
use rayon::prelude::*;

type Input = String;

#[aoc_generator(day22)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

fn round(mut secret: i64) -> i64 {
    secret ^= secret << 6;
    secret &= 0xFFFFFF;
    secret ^= secret >> 5;
    secret &= 0xFFFFFF;
    secret ^= secret << 11;
    secret & 0xFFFFFF
}

pub fn chunk_round<const C: usize>(secret: &[i32; C]) -> [i32; C] {
    // secret.map(|mut secret| {
    //     secret ^= secret << 6;
    //     secret &= 0xFFFFFF;
    //     secret ^= secret >> 5;
    //     secret &= 0xFFFFFF;
    //     secret ^= secret << 11;
    //     secret & 0xFFFFFF
    // })
    secret.map(|s| s + 30)
}

pub fn chunk_round_8(secret: &[i32; 32]) -> [i32; 32] {
    chunk_round(secret)
}

fn round2000(mut secret: i64) -> i64 {
    for _ in 0..2000 {
        secret = round(secret);
    }
    secret
}

fn chunk_round2000<const C: usize>(mut secret: [i32; C]) -> [i32; C] {
    for _ in 0..2000 {
        secret = chunk_round(&secret);
    }
    secret
}

fn prices<const N: usize>(mut secret: i64) -> [i8; N] {
    core::array::from_fn(|_| {
        secret = round(secret);
        (secret % 10) as i8
    })
}

fn changes<const N: usize>(prev: i64, prices: &[i8; N]) -> [i8; N] {
    let mut prev = (prev % 10) as i8;
    prices.map(|v| {
        let diff = v - prev;
        prev = v;
        diff
    })
}

#[aoc(day22, part1)]
fn solver_part1(input: &Input) -> i64 {
    input
        .lines()
        .map(|line| {
            let secret = line.parse::<u32>().unwrap() as i64;
            round2000(secret)
        })
        .sum()
}

#[aoc(day22, part1, BULK)]
fn solver_part1_bulk(input: &Input) -> i64 {
    let secrets = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap() as i64)
        .collect_vec();
    secrets.into_iter().map(round2000).sum()
}

#[aoc(day22, part1, CHUNK)]
fn solver_part1_chunk(input: &Input) -> i64 {
    let secrets = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap() as i32)
        .collect_vec();

    const C: usize = 16;
    let mut iter = secrets[0..].chunks_exact(C);

    let mut sum = 0;
    for c in &mut iter {
        sum += chunk_round2000(<[_; C]>::try_from(c).unwrap())
            .into_iter()
            .map(|v| v as i64)
            .sum::<i64>();
    }

    sum + iter
        .remainder()
        .iter()
        .map(|secret| round2000(*secret as i64))
        .sum::<i64>()
}

#[aoc(day22, part1, CHUNK_PARALLEL)]
fn solver_part1_chunk_parallel(input: &Input) -> i64 {
    let secrets = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap() as i32)
        .collect_vec();

    const C: usize = 32;
    let iter = secrets[0..].par_chunks_exact(C);

    let sum = iter
        .remainder()
        .iter()
        .map(|secret| round2000(*secret as i64))
        .sum::<i64>();

    iter.map(|c| {
        chunk_round2000(<[_; C]>::try_from(c).unwrap())
            .into_iter()
            .map(|v| v as i64)
            .sum::<i64>()
    })
    .sum::<i64>()
        + sum
}

// #[aoc(day22, part1, TEST)]
// #[inline(never)]
// fn solver_part1_test(input: &Input) -> u32 {
//     let vec = (0..(input.len() as u32)).collect_vec();
//     vec[0..1024]
//         .chunks_exact(8)
//         .map(|c| c.iter().map(|v| v.pow(2)).sum::<u32>())
//         .sum()
// }

struct SeqIter {
    ind: usize,
    next: [i8; 4],
}

impl SeqIter {
    fn new() -> Self {
        SeqIter {
            ind: 3,
            next: [-9, -9, -9, -10],
        }
    }
}

impl SeqIter {
    fn inc(&mut self) -> Option<[i8; 4]> {
        self.next[self.ind] += 1;
        if self.next[self.ind] == 10 {
            if self.ind == 0 {
                None
            } else {
                self.next[self.ind] = -9;
                self.ind -= 1;
                self.inc()
            }
        } else {
            self.ind = 3;
            Some(self.next)
        }
    }
}

impl Iterator for SeqIter {
    type Item = [i8; 4];

    fn next(&mut self) -> Option<Self::Item> {
        self.inc()
    }
}

fn get_bananas<const N: usize>(seq: &[i8; 4], change: &[i8; N], prices: &[i8; N]) -> Option<i8> {
    change
        .windows(5)
        .zip(prices[3..].iter())
        .find(|(w, _)| w[0..4].iter().eq(seq))
        .map(|(_, p)| *p)
}

#[aoc(day22, part2)]
fn solver_part2(input: &Input) -> i32 {
    let secrets = input
        .lines()
        .map(|line| line.parse::<u32>().unwrap() as i64)
        .collect_vec();
    let c = secrets
        .into_iter()
        .map(|secret| {
            let p = prices(secret);
            (changes::<2000>(secret, &p), p)
        })
        .collect_vec();
    // dbg!(c);
    SeqIter::new()
        .par_bridge()
        .map(|seq| {
            (
                c.iter()
                    .flat_map(|(c, p)| get_bananas(&seq, c, p).map(|v| v as i32))
                    .sum::<i32>(),
                seq,
            )
        })
        .max_by_key(|(v, _)| *v)
        .unwrap()
        .0
}

#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "1
10
100
2024";

    static INPUT2: &str = "1
2
3
2024";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 37327623)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT2)), 23)
    }
}

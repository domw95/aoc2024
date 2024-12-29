use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Either;
use itertools::Itertools;

type Input = String;

#[derive(Debug, Clone, Copy)]
struct Key([u8; 5]);

#[derive(Debug, Clone, Copy)]
struct Lock([u8; 5]);

impl Lock {
    fn overlap(&self, key: &Key) -> bool {
        self.0.iter().zip(key.0.iter()).any(|(l, k)| l + k >= 6)
    }
}
#[aoc_generator(day25)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

#[aoc(day25, part1)]
fn solver_part1(input: &Input) -> usize {
    let (locks, keys): (Vec<_>, Vec<_>) =
        input
            .lines()
            .chunks(8)
            .into_iter()
            .partition_map(|mut chunk| {
                let mut arr = [0; 5];
                let lock = &chunk.next().unwrap()[0..1] == "#";
                // lock
                for line in chunk.take(5) {
                    for (c, a) in line.as_bytes().iter().zip(arr.iter_mut()) {
                        if *c == b'#' {
                            *a += 1
                        }
                    }
                }
                if lock {
                    Either::Left(Lock(arr))
                } else {
                    Either::Right(Key(arr))
                }
            });
    locks
        .iter()
        .cartesian_product(keys.iter())
        .filter(|(l, k)| !l.overlap(k))
        .count()
}

#[aoc(day25, part2)]
fn solver_part2(_input: &Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 3)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 0)
    }
}

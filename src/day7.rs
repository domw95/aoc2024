use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;
use rayon::prelude::*;

type Input = String;

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mult,
}

#[derive(Debug, Clone, Copy)]
enum Op2 {
    Add,
    Mult,
    Concat,
}

fn op_iter(len: usize) -> itertools::MultiProduct<std::slice::Iter<'static, Op>> {
    (0..len)
        .map(|_| [Op::Add, Op::Mult].iter())
        .multi_cartesian_product()
}

fn op2_iter(len: usize) -> itertools::MultiProduct<std::slice::Iter<'static, Op2>> {
    (0..len)
        .map(|_| [Op2::Add, Op2::Mult, Op2::Concat].iter())
        .multi_cartesian_product()
}

#[aoc_generator(day7)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

#[aoc(day7, part1)]
fn solver_part1(input: &Input) -> u64 {
    input
        .lines()
        .filter_map(|l| {
            let s = l.split_once(": ").unwrap();
            let sum: u64 = s.0.parse().unwrap();
            let values: Vec<u64> = s.1.split(' ').map(|s| s.parse::<u64>().unwrap()).collect();
            if op_iter(values.len() - 1).any(|ops| {
                values[1..]
                    .iter()
                    .zip(ops.into_iter())
                    .fold(values[0], |total, (v, op)| match op {
                        Op::Add => total + v,
                        Op::Mult => total * v,
                    })
                    == sum
            }) {
                Some(sum)
            } else {
                None
            }
        })
        .sum()
}

fn recursive(total: u64, index: usize, vec: &[u64]) -> bool {
    if index == 0 {
        total == vec[0]
    } else {
        let v = vec[index];
        if total % v == 0 && recursive(total / v, index - 1, vec) {
            return true;
        }
        if total > v && recursive(total - v, index - 1, vec) {
            return true;
        }
        false
    }
}

fn recursive_switch(total: u64, index: usize, vec: &[u64]) -> bool {
    if index == 0 {
        total == vec[0]
    } else {
        let v = vec[index];
        if total > v && recursive_switch(total - v, index - 1, vec) {
            return true;
        }
        if total % v == 0 && recursive_switch(total / v, index - 1, vec) {
            return true;
        }
        false
    }
}

fn recursive_2(total: u64, index: usize, vec: &[u64]) -> bool {
    if index == 0 {
        total == vec[0]
    } else {
        let v = vec[index];
        if v < 10 {
            if total.rem_euclid(10) == v && recursive_2(total / 10, index - 1, vec) {
                return true;
            }
        } else if v < 100 {
            if total.rem_euclid(100) == v && recursive_2(total / 100, index - 1, vec) {
                return true;
            }
        } else if v < 1000
            && total.rem_euclid(1000) == v
            && recursive_2(total / 1000, index - 1, vec)
        {
            return true;
        }
        if total % v == 0 && recursive_2(total / v, index - 1, vec) {
            return true;
        }
        if total > v && recursive_2(total - v, index - 1, vec) {
            return true;
        }
        false
    }
}
#[aoc(day7, part1, RECURSIVE)]
fn solver_part1_recursive(input: &Input) -> u64 {
    input
        .lines()
        .filter_map(|l| {
            let s = l.split_once(": ").unwrap();
            let sum: u64 = s.0.parse().unwrap();
            let values: Vec<u64> = s.1.split(' ').map(|s| s.parse::<u64>().unwrap()).collect();

            if recursive(sum, values.len() - 1, &values) {
                Some(sum)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day7, part1, RECURSIVE_PARALLEL)]
fn solver_part1_recursive_paralell(input: &Input) -> u64 {
    input
        .lines()
        .par_bridge()
        .filter_map(|l| {
            let s = l.split_once(": ").unwrap();
            let sum: u64 = s.0.parse().unwrap();
            let values: Vec<u64> = s.1.split(' ').map(|s| s.parse::<u64>().unwrap()).collect();

            if recursive(sum, values.len() - 1, &values) {
                Some(sum)
            } else {
                None
            }
        })
        .sum()
}
#[aoc(day7, part1, RECURSIVE_SWITCH)]
fn solver_part1_recursive_switch(input: &Input) -> u64 {
    input
        .lines()
        .filter_map(|l| {
            let s = l.split_once(": ").unwrap();
            let sum: u64 = s.0.parse().unwrap();
            let values: Vec<u64> = s.1.split(' ').map(|s| s.parse::<u64>().unwrap()).collect();

            if recursive_switch(sum, values.len() - 1, &values) {
                Some(sum)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day7, part2)]
fn solver_part2(input: &Input) -> u64 {
    input
        .lines()
        .filter_map(|l| {
            let s = l.split_once(": ").unwrap();
            let sum: u64 = s.0.parse().unwrap();
            let values: Vec<u64> = s.1.split(' ').map(|s| s.parse::<u64>().unwrap()).collect();
            if op2_iter(values.len() - 1).any(|ops| {
                values[1..]
                    .iter()
                    .zip(ops.into_iter())
                    .fold(values[0], |total, (v, op)| match op {
                        Op2::Add => total + v,
                        Op2::Mult => total * v,
                        Op2::Concat => (total.to_string() + &v.to_string()).parse().unwrap(),
                    })
                    == sum
            }) {
                Some(sum)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day7, part2, FAST_CONCAT)]
fn solver_part2_fast_concat(input: &Input) -> u64 {
    input
        .lines()
        .filter_map(|l| {
            let s = l.split_once(": ").unwrap();
            let sum: u64 = s.0.parse().unwrap();
            let values: Vec<u64> = s.1.split(' ').map(|s| s.parse::<u64>().unwrap()).collect();
            if op2_iter(values.len() - 1).any(|ops| {
                values[1..]
                    .iter()
                    .zip(ops.into_iter())
                    .fold(values[0], |total, (v, op)| match op {
                        Op2::Add => total + v,
                        Op2::Mult => total * v,
                        Op2::Concat => total * 10u64.pow(v.to_string().len() as u32) + v,
                    })
                    == sum
            }) {
                Some(sum)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day7, part2, FASTER_CONCAT)]
fn solver_part2_faster_concat(input: &Input) -> u64 {
    input
        .lines()
        .filter_map(|l| {
            let s = l.split_once(": ").unwrap();
            let sum: u64 = s.0.parse().unwrap();
            let values: Vec<u64> = s.1.split(' ').map(|s| s.parse::<u64>().unwrap()).collect();
            if op2_iter(values.len() - 1).any(|ops| {
                values[1..].iter().zip(ops.into_iter()).fold(
                    values[0],
                    |total, (&v, op)| match op {
                        Op2::Add => total + v,
                        Op2::Mult => total * v,
                        Op2::Concat => {
                            let pow = if v < 10 {
                                1
                            } else if v < 100 {
                                2
                            } else if v < 1000 {
                                3
                            } else {
                                4
                            };
                            total * 10u64.pow(pow) + v
                        }
                    },
                ) == sum
            }) {
                Some(sum)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day7, part2, RECURSIVE)]
fn solver_part2_recursive(input: &Input) -> u64 {
    input
        .lines()
        .filter_map(|l| {
            let s = l.split_once(": ").unwrap();
            let sum: u64 = s.0.parse().unwrap();
            let values: Vec<u64> = s.1.split(' ').map(|s| s.parse::<u64>().unwrap()).collect();

            if recursive_2(sum, values.len() - 1, &values) {
                Some(sum)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 3749)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 11387)
    }
}

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Input = String;

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

fn parse_fast(input: &str) -> Vec<Vec<i16>> {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|v| v.parse::<u8>().unwrap() as i16)
                .collect_vec()
        })
        .collect_vec()
}

fn parse_custom(input: &str) -> Vec<Vec<i16>> {
    input.lines().map(line2vec).collect_vec()
}

fn line2vec(l: &str) -> Vec<i16> {
    let mut vec = Vec::with_capacity(l.len() / 3);
    // let mut vec = Vec::new();
    let mut number = 0;

    for b in l.bytes() {
        match b {
            32 => {
                vec.push(number as i16);
                number = 0;
            }
            x => {
                number *= 10;
                number += x - 48;
            }
        }
    }
    vec.push(number as i16);
    vec
}

#[aoc(day2, part1)]
fn solver_part1(input: &Input) -> usize {
    let input = parse(input);
    input
        .iter()
        .filter(|vec| {
            //
            let diff = vec.iter().tuple_windows().map(|(a, b)| b - a).collect_vec();
            let sign = diff.iter().map(|v| v.signum()).collect_vec();
            let mag = diff.iter().map(|v| v.abs()).collect_vec();
            mag.iter().all(|v| *v <= 3 && *v >= 1) && sign.iter().all_equal()
        })
        .count()
}

#[aoc(day2, part1, FAST)]
fn solver_part1_fast(input: &Input) -> usize {
    let input = parse(input);
    input
        .iter()
        .filter(|vec| {
            //
            let sign = (vec[1] - vec[0]).signum();
            vec.iter().tuple_windows().all(|(a, b)| {
                let abs = (b - a).abs();
                (1..=3).contains(&abs) && (b - a).signum() == sign
            })
        })
        .count()
}

#[aoc(day2, part1, FAST_PARSE)]
fn solver_part1_fast_parse(input: &Input) -> usize {
    let input = parse_fast(input);
    input
        .iter()
        .filter(|vec| {
            //
            let sign = (vec[1] - vec[0]).signum();
            vec.iter().tuple_windows().all(|(a, b)| {
                let abs = (b - a).abs();
                (1..=3).contains(&abs) && (b - a).signum() == sign
            })
        })
        .count()
}

#[aoc(day2, part1, FAST_PARSE_CUSTOM)]
fn solver_part1_fast_parse_custom(input: &Input) -> usize {
    let input = parse_custom(input);
    input
        .iter()
        .filter(|vec| {
            //
            let sign = (vec[1] - vec[0]).signum();
            vec.iter().tuple_windows().all(|(a, b)| {
                let abs = (b - a).abs();
                (1..=3).contains(&abs) && (b - a).signum() == sign
            })
        })
        .count()
}

#[aoc(day2, part1, FAST_INLINE)]
fn solver_part1_fast_inline(input: &Input) -> usize {
    input
        .lines()
        .filter(|l| {
            //
            let vec = line2vec(l);
            let sign = vec[0].signum();
            vec.iter()
                .all(|diff| diff.signum() == sign && (1..=3).contains(&diff.abs()))
        })
        .count()
}

#[aoc(day2, part1, FAST_INLINE_2)]
fn solver_part1_fast_inline_2(input: &Input) -> usize {
    input
        .lines()
        .filter(|l| {
            //
            let vec = line2vec(l);
            // let sign = vec[0].signum();
            vec.iter().all(|diff| (1..=3).contains(diff))
                || vec.iter().all(|diff| (-3..=-1).contains(diff))
        })
        .count()
}

#[aoc(day2, part2)]
fn solver_part2(input: &Input) -> usize {
    let input = parse(input);
    input
        .iter()
        .filter(|vec| {
            //

            vec.iter().combinations(vec.len() - 1).any(|vec| {
                let diff = vec
                    .iter()
                    .tuple_windows()
                    .map(|(a, b)| **b - **a)
                    .collect_vec();
                let sign = diff.iter().map(|v| v.signum()).collect_vec();
                let mag = diff.iter().map(|v| v.abs()).collect_vec();
                mag.iter().all(|v| *v <= 3 && *v >= 1) && sign.iter().all_equal()
            })
        })
        .count()
}

#[aoc(day2, part2, FAST)]
fn solver_part2_fast(input: &Input) -> usize {
    let input = parse(input);
    input
        .iter()
        .filter(|vec| {
            //

            vec.iter().combinations(vec.len() - 1).any(|vec| {
                let sign = (vec[1] - vec[0]).signum();
                vec.into_iter().tuple_windows().all(|(a, b)| {
                    let abs = (*b - *a).abs();
                    (1..=3).contains(&abs) && (*b - *a).signum() == sign
                })
            })
        })
        .count()
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

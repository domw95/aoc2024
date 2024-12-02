use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Input = Vec<Vec<i32>>;

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect_vec()
        })
        .collect_vec()
}

#[aoc(day2, part1)]
fn solver_part1(input: &Input) -> usize {
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

#[aoc(day2, part2)]
fn solver_part2(input: &Input) -> usize {
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

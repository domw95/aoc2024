use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use itertools::Itertools;

type Input = Vec<[(u32, u32); 3]>;

fn parse_button(str: &str) -> (u32, u32) {
    (str[12..14].parse().unwrap(), str[18..20].parse().unwrap())
}

fn parse_prize(str: &str) -> (u32, u32) {
    (
        str[9..].split_once(',').unwrap().0.parse().unwrap(),
        str[9..].split_once('=').unwrap().1.parse().unwrap(),
    )
}
#[aoc_generator(day13)]
fn input_generator(input: &str) -> Input {
    input
        .lines()
        .chunks(4)
        .into_iter()
        .map(|mut chunk| {
            let mut arr = [(0, 0); 3];
            arr[0] = parse_button(chunk.next().unwrap());
            arr[1] = parse_button(chunk.next().unwrap());
            arr[2] = parse_prize(chunk.next().unwrap());
            arr
        })
        .collect()
}

#[aoc(day13, part1)]
fn solver_part1(input: &Input) -> u32 {
    input
        .iter()
        .filter_map(|arr| {
            let A = nalgebra::Matrix2::new(arr[0].0, arr[1].0, arr[0].1, arr[1].1);
            let Af = A.map(|i| i as f64);
            let B = nalgebra::Matrix2x1::new(arr[2].0, arr[2].1);
            let Bf = B.map(|i| i as f64);

            match Af.try_inverse() {
                Some(inv) => {
                    let res = (inv * Bf).map(|i| i.round() as u32);
                    if A * res == B {
                        Some((res[0] * 3) + res[1])
                    } else {
                        None
                    }
                }
                None => {
                    dbg!(A);
                    None
                }
            }
        })
        .sum()
}

#[aoc(day13, part2)]
fn solver_part2(input: &Input) -> u64 {
    input
        .iter()
        .filter_map(|arr| {
            let A =
                nalgebra::Matrix2::new(arr[0].0, arr[1].0, arr[0].1, arr[1].1).map(|i| i as u64);
            let Af = A.map(|i| i as f64);
            let B = nalgebra::Matrix2x1::new(arr[2].0, arr[2].1).map(|i| i as u64 + 10000000000000);
            let Bf = B.map(|i| i as f64);

            match Af.try_inverse() {
                Some(inv) => {
                    let res = (inv * Bf).map(|i| i.round() as u64);
                    if A * res == B {
                        Some((res[0] * 3) + res[1])
                    } else {
                        None
                    }
                }
                None => None,
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 480)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 875318608908)
    }
}

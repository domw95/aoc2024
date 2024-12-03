use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

type Input = String;

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

#[aoc(day3, part1)]
fn solver_part1(input: &Input) -> u32 {
    let re = regex::Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
    re.captures_iter(input)
        .map(|c| c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap())
        .sum()
}

#[aoc(day3, part2)]
fn solver_part2(input: &Input) -> u32 {
    let re = regex::Regex::new(r"mul\(\d\d?\d?,\d\d?\d?\)|do\(\)|don't\(\)").unwrap();
    let re_mul = regex::Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
    let mut enable = true;
    let mut sum = 0;
    for m in re.find_iter(input) {
        if m.as_str() == "do()" {
            enable = true
        } else if m.as_str() == "don't()" {
            enable = false
        } else if enable {
            let cap = re_mul.captures(m.as_str()).unwrap();
            sum += cap[1].parse::<u32>().unwrap() * cap[2].parse::<u32>().unwrap();
        }
    }
    sum
}

#[aoc(day3, part2, SINGLE)]
fn solver_part2_single(input: &Input) -> u32 {
    let re = regex::Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)|do\(\)|don't\(\)").unwrap();
    let mut enable = true;
    let mut sum = 0;
    for c in re.captures_iter(input) {
        if c[0].len() == 4 {
            enable = true
        } else if c[0].len() == 7 {
            enable = false
        } else if enable {
            sum += c[1].parse::<u32>().unwrap() * c[2].parse::<u32>().unwrap();
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::day3::solver_part2;

    use super::input_generator;
    use super::solver_part1;

    static INPUT: &str = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    static INPUT2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn sample1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 161)
    }

    #[test]
    fn sample2() {
        assert_eq!(solver_part2(&input_generator(INPUT2)), 48)
    }
}

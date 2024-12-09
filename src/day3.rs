use std::str::Bytes;

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

#[derive(Debug)]
enum State {
    M,
    U,
    L,
    Lbracket,
    V1,
    V2,
}

#[aoc(day3, part1, DFA)]
fn solver_part1_dfa(input: &Input) -> u32 {
    // let re = regex::Regex::new(r"mul\((\d\d?\d?),(\d\d?\d?)\)").unwrap();
    let mut v1 = 0;
    let mut v2 = 0;
    let mut sum = 0;
    let mut digit_count = 0;
    let mut state = State::M;
    for b in input.bytes() {
        match (&state, b) {
            (State::M, b'm') => state = State::U,
            (State::U, b'u') => state = State::L,
            (State::L, b'l') => state = State::Lbracket,
            (State::Lbracket, b'(') => state = State::V1,
            (State::V1, _) => {
                // println!("V1");
                if b.is_ascii_digit() {
                    digit_count += 1;
                    if digit_count == 4 {
                        digit_count = 0;
                        state = State::M;
                        v1 = 0;
                        continue;
                    }
                    v1 *= 10;
                    v1 += b as u32 - 48;
                } else if b == b',' && digit_count != 0 {
                    state = State::V2;
                    digit_count = 0;
                } else {
                    digit_count = 0;
                    state = State::M;
                    v1 = 0;
                }
            }
            (State::V2, _) => {
                // println!("V2");
                if b.is_ascii_digit() {
                    digit_count += 1;
                    if digit_count == 4 {
                        digit_count = 0;
                        state = State::M;
                        v2 = 0;
                        continue;
                    }
                    v2 *= 10;
                    v2 += b as u32 - 48;
                } else if b == b')' && digit_count != 0 {
                    sum += v1 * v2;
                    digit_count = 0;
                    v1 = 0;
                    v2 = 0;
                    state = State::M;
                } else {
                    digit_count = 0;
                    state = State::M;
                    v1 = 0;
                    v2 = 0;
                }
            }

            _ => {
                state = State::M;
                v1 = 0;
                v2 = 0;
            }
        }
    }
    sum
}

fn find_mul(bytes: &mut Bytes<'_>) -> Option<u32> {
    loop {
        if bytes.next()? == b'm'
            && bytes.next()? == b'u'
            && bytes.next()? == b'l'
            && bytes.next()? == b'('
        {
            let mut v1 = 0;

            let mut b = bytes.next()?;
            if b.is_ascii_digit() {
                v1 += b as u32 - 48;
                b = bytes.next()?;
                if b.is_ascii_digit() {
                    v1 *= 10;
                    v1 += b as u32 - 48;
                    b = bytes.next()?;
                    if b.is_ascii_digit() {
                        v1 *= 10;
                        v1 += b as u32 - 48;
                        b = bytes.next()?;
                    }
                }
            } else {
                continue;
            }
            if b != b',' {
                continue;
            }
            let mut v2 = 0;

            let mut b = bytes.next()?;
            if b.is_ascii_digit() {
                v2 += b as u32 - 48;
                b = bytes.next()?;
                if b.is_ascii_digit() {
                    v2 *= 10;
                    v2 += b as u32 - 48;
                    b = bytes.next()?;
                    if b.is_ascii_digit() {
                        v2 *= 10;
                        v2 += b as u32 - 48;
                        b = bytes.next()?;
                    }
                }
            } else {
                continue;
            }
            if b != b')' {
                continue;
            }
            return Some(v1 * v2);
        }
    }
}

#[aoc(day3, part1, DFA2)]
fn solver_part1_dfa2(input: &Input) -> u32 {
    let mut sum = 0;
    let mut bytes = input.bytes();
    while let Some(v) = find_mul(&mut bytes) {
        sum += v;
    }
    sum
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

fn find_mul_part2(bytes: &mut Bytes<'_>, active: &mut bool) -> Option<u32> {
    loop {
        let b = bytes.next()?;
        if *active
            && b == b'm'
            && bytes.next()? == b'u'
            && bytes.next()? == b'l'
            && bytes.next()? == b'('
        {
            let mut v1 = 0;

            let mut b = bytes.next()?;
            if b.is_ascii_digit() {
                v1 += b as u32 - 48;
                b = bytes.next()?;
                if b.is_ascii_digit() {
                    v1 *= 10;
                    v1 += b as u32 - 48;
                    b = bytes.next()?;
                    if b.is_ascii_digit() {
                        v1 *= 10;
                        v1 += b as u32 - 48;
                        b = bytes.next()?;
                    }
                }
            } else {
                continue;
            }
            if b != b',' {
                continue;
            }
            let mut v2 = 0;

            let mut b = bytes.next()?;
            if b.is_ascii_digit() {
                v2 += b as u32 - 48;
                b = bytes.next()?;
                if b.is_ascii_digit() {
                    v2 *= 10;
                    v2 += b as u32 - 48;
                    b = bytes.next()?;
                    if b.is_ascii_digit() {
                        v2 *= 10;
                        v2 += b as u32 - 48;
                        b = bytes.next()?;
                    }
                }
            } else {
                continue;
            }
            if b != b')' {
                continue;
            }
            return Some(v1 * v2);
        } else if b == b'd' && bytes.next()? == b'o' {
            let b = bytes.next()?;
            if !*active && b == b'(' && bytes.next()? == b')' {
                *active = true;
            } else if *active
                && b == b'n'
                && bytes.next()? == b'\''
                && bytes.next()? == b't'
                && bytes.next()? == b'('
                && bytes.next()? == b')'
            {
                *active = false
            }
        }
    }
}

#[aoc(day3, part2, DFA)]
fn solver_part2_dfa(input: &Input) -> u32 {
    let mut sum = 0;
    let mut bytes = input.bytes();
    let mut active = true;
    while let Some(v) = find_mul_part2(&mut bytes, &mut active) {
        sum += v;
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::day3::solver_part1_dfa;
    use crate::day3::solver_part1_dfa2;
    use crate::day3::solver_part2;
    use crate::day3::solver_part2_dfa;

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
    fn part1_dfa() {
        assert_eq!(solver_part1_dfa(&input_generator(INPUT)), 161)
    }

    #[test]
    fn part1_dfa2() {
        assert_eq!(solver_part1_dfa2(&input_generator(INPUT)), 161)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT2)), 48)
    }

    #[test]
    fn part2_dfa() {
        assert_eq!(solver_part2_dfa(&input_generator(INPUT2)), 48)
    }
}

use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;

type Input = String;

#[aoc_generator(day17)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

#[derive(Debug, Clone, Copy)]
struct Registers {
    a: u64,
    b: u64,
    c: u64,
}

impl Registers {
    fn new(a: u64, b: u64, c: u64) -> Self {
        Self { a, b, c }
    }
}

fn get_combo(reg: &Registers, code: u8) -> u64 {
    match code {
        x @ 0..=3 => x as u64,
        4 => reg.a,
        5 => reg.b,
        6 => reg.c,
        _ => 0,
    }
}

struct OutputIter {
    index: usize,
    ins: Vec<u8>,
    reg: Registers,
}

impl OutputIter {
    fn new(reg: Registers, ins: Vec<u8>) -> OutputIter {
        Self { index: 0, ins, reg }
    }
}

impl Iterator for OutputIter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if self.index >= self.ins.len() {
                return None;
            }
            match self.ins[self.index] {
                0 => self.reg.a >>= get_combo(&self.reg, self.ins[self.index + 1]),
                1 => self.reg.b ^= self.ins[self.index + 1] as u64,
                2 => self.reg.b = get_combo(&self.reg, self.ins[self.index + 1]) & 0b111,
                3 => {
                    if self.reg.a != 0 {
                        self.index = self.ins[self.index + 1] as usize;
                        continue;
                    }
                }
                4 => self.reg.b ^= self.reg.c,
                5 => {
                    let res = Some((get_combo(&self.reg, self.ins[self.index + 1]) & 0b111) as u8);
                    self.index += 2;
                    return res;
                }
                6 => self.reg.b = self.reg.a >> get_combo(&self.reg, self.ins[self.index + 1]),
                7 => self.reg.c = self.reg.a >> get_combo(&self.reg, self.ins[self.index + 1]),

                _ => (),
            }
            self.index += 2
        }
    }
}
fn parse_input(input: &str) -> (Registers, Vec<u8>) {
    let mut lines = input.lines();
    let reg_a = lines.next().unwrap()[12..].parse::<u64>().unwrap();
    let reg_b = lines.next().unwrap()[12..].parse::<u64>().unwrap();
    let reg_c = lines.next().unwrap()[12..].parse::<u64>().unwrap();
    let reg = Registers::new(reg_a, reg_b, reg_c);

    lines.next();
    let ins = lines.next().unwrap()[9..]
        .split(',')
        .map(|str| str.parse::<u8>().unwrap())
        .collect::<Vec<_>>();
    (reg, ins)
}

fn fast_parse_input(input: &str) -> (Registers, Vec<u8>) {
    let mut lines = input.lines();
    let reg_a = lines.next().unwrap()[12..].parse::<u64>().unwrap();
    let reg_b = lines.next().unwrap()[12..].parse::<u64>().unwrap();
    let reg_c = lines.next().unwrap()[12..].parse::<u64>().unwrap();
    let reg = Registers::new(reg_a, reg_b, reg_c);

    lines.next();
    let ins = lines.next().unwrap()[9..]
        .split(',')
        .map(|str| str.as_bytes()[0] - 48)
        .collect::<Vec<_>>();
    (reg, ins)
}

fn faster_parse_input(input: &str) -> (Registers, Vec<u8>) {
    let mut lines = input.lines();
    let reg_a = lines.next().unwrap()[12..].parse::<u64>().unwrap();
    let reg_b = lines.next().unwrap()[12..].parse::<u64>().unwrap();
    let reg_c = lines.next().unwrap()[12..].parse::<u64>().unwrap();
    let reg = Registers::new(reg_a, reg_b, reg_c);

    lines.next();
    let ins = lines.next().unwrap()[9..]
        .bytes()
        .step_by(2)
        .map(|v| v - 48)
        .collect::<Vec<_>>();
    (reg, ins)
}

fn faster_parse_ins(input: &str) -> Vec<u8> {
    let mut lines = input.lines();

    lines.next();
    lines.next();
    lines.next();
    lines.next();
    let ins = lines.next().unwrap()[9..]
        .bytes()
        .step_by(2)
        .map(|v| v - 48)
        .collect::<Vec<_>>();
    ins
}

fn run_program(reg: &mut Registers, ins: &[u8]) -> Vec<u8> {
    let mut index = 0;
    let mut out = Vec::new();
    loop {
        if index >= ins.len() {
            break;
        }
        match ins[index] {
            0 => reg.a /= 2u64.pow(get_combo(reg, ins[index + 1]) as u32),
            1 => reg.b ^= ins[index + 1] as u64,
            2 => reg.b = get_combo(reg, ins[index + 1]) & 0b111,
            3 => {
                if reg.a != 0 {
                    index = ins[index + 1] as usize;
                    continue;
                }
            }
            4 => reg.b ^= reg.c,
            5 => out.push((get_combo(reg, ins[index + 1]) & 0b111) as u8),
            6 => reg.b = reg.a / (2u64.pow(get_combo(reg, ins[index + 1]) as u32)),
            7 => reg.c = reg.a / (2u64.pow(get_combo(reg, ins[index + 1]) as u32)),

            _ => (),
        }
        index += 2
    }
    out
}

fn run_program_shift(reg: &mut Registers, ins: &[u8]) -> Vec<u8> {
    let mut index = 0;
    let mut out = Vec::new();
    loop {
        if index >= ins.len() {
            break;
        }
        match ins[index] {
            0 => reg.a >>= get_combo(reg, ins[index + 1]),
            1 => reg.b ^= ins[index + 1] as u64,
            2 => reg.b = get_combo(reg, ins[index + 1]) & 0b111,
            3 => {
                if reg.a != 0 {
                    index = ins[index + 1] as usize;
                    continue;
                }
            }
            4 => reg.b ^= reg.c,
            5 => out.push((get_combo(reg, ins[index + 1]) & 0b111) as u8),
            6 => reg.b = reg.a >> get_combo(reg, ins[index + 1]),
            7 => reg.c = reg.a >> get_combo(reg, ins[index + 1]),

            _ => (),
        }
        index += 2
    }
    out
}
#[aoc(day17, part1)]
fn solver_part1(input: &Input) -> String {
    let (mut reg, ins) = parse_input(input);

    let out = run_program(&mut reg, &ins);
    let mut string = String::new();
    for v in out {
        string.push((v + 48) as char);
        string.push(',');
    }
    string.pop();
    string
}

#[aoc(day17, part1, ITER)]
fn solver_part1_iter(input: &Input) -> String {
    let (reg, ins) = parse_input(input);
    let iter = OutputIter::new(reg, ins);
    let mut string = String::new();
    for v in iter {
        string.push((v + 48) as char);
        string.push(',');
    }
    string.pop();
    string
}

#[aoc(day17, part1, SHIFT)]
fn solver_part1_shift(input: &Input) -> String {
    let (mut reg, ins) = parse_input(input);

    let out = run_program_shift(&mut reg, &ins);
    let mut string = String::new();
    for v in out {
        string.push((v + 48) as char);
        string.push(',');
    }
    string.pop();
    string
}

#[aoc(day17, part1, SHIFT_FAST_PARSE)]
fn solver_part1_shift_fast_parse(input: &Input) -> String {
    let (mut reg, ins) = fast_parse_input(input);

    let out = run_program_shift(&mut reg, &ins);
    let mut string = String::new();
    for v in out {
        string.push((v + 48) as char);
        string.push(',');
    }
    string.pop();
    string
}

#[aoc(day17, part1, SHIFT_FASTER_PARSE)]
fn solver_part1_shift_faster_parse(input: &Input) -> String {
    let (mut reg, ins) = faster_parse_input(input);

    let out = run_program_shift(&mut reg, &ins);
    let mut string = String::new();
    for v in out {
        string.push((v + 48) as char);
        string.push(',');
    }
    string.pop();
    string
}

fn find_next(mut reg: Registers, ins: &[u8], index: usize, ins_index: usize) -> Option<u64> {
    if index == ins.len() {
        let a = reg.a;
        reg.a = a >> (ins_index * 3);
        let iter = OutputIter::new(reg, ins.to_vec());

        if iter.eq(ins[ins_index..].iter().copied()) {
            Some(a)
        } else {
            None
        }
    } else {
        for i in 0..8u64 {
            let mut reg = reg;
            let a = reg.a + (i << (index * 3));
            reg.a = a >> (ins_index * 3);
            if OutputIter::new(reg, ins.to_vec()).next().unwrap() == ins[ins_index] {
                reg.a = a;
                if let Some(res) = find_next(reg, ins, index + 1, ins_index + 1) {
                    return Some(res);
                }
            }
        }
        None
    }
}

fn step_fast(mut reg: Registers, ins: &[u8]) -> u8 {
    for index in (0..ins.len()).step_by(2) {
        match ins[index] {
            0 => reg.a >>= get_combo(&reg, ins[index + 1]),
            1 => reg.b ^= ins[index + 1] as u64,
            2 => reg.b = get_combo(&reg, ins[index + 1]) & 0b111,
            3 => (),
            4 => reg.b ^= reg.c,
            5 => return (get_combo(&reg, ins[index + 1]) & 0b111) as u8,
            6 => reg.b = reg.a >> get_combo(&reg, ins[index + 1]),
            7 => reg.c = reg.a >> get_combo(&reg, ins[index + 1]),

            _ => (),
        }
    }
    0
}

#[aoc(day17, part2)]
fn solver_part2(input: &Input) -> u64 {
    let (_, ins) = faster_parse_input(input);
    for i in 0..512 {
        let reg = Registers::new(i, 0, 0);
        if let Some(v) = find_next(reg, &ins, 3, 0) {
            return v;
        }
    }
    0
}

#[aoc(day17, part2, BACKWARDS)]
fn solver_part2_backwrds(input: &Input) -> u64 {
    let (_, ins) = faster_parse_input(input);
    let mut values = vec![0u64; ins.len()];
    let mut index = ins.len() - 1;
    let mut a = 0u64;
    loop {
        let a_t = a + values[index];
        // println!("{values:?} : {index},{a}:{a_t}");
        if OutputIter::new(Registers::new(a_t, 0, 0), ins.to_vec())
            .next()
            .unwrap()
            == ins[index]
        {
            if index == 0 {
                return a_t;
            } else {
                index -= 1;
                a = a_t;
                a <<= 3;
            }
        } else {
            values[index] += 1;
            while values[index] == 8 {
                values[index] = 0;
                index += 1;
                a >>= 3;
                a &= !0b111;

                values[index] += 1;
                // a += 1;
            }
        }
    }
}

#[aoc(day17, part2, BACKWARDS_FASTER)]
fn solver_part2_backwrds_faster(input: &Input) -> u64 {
    let ins = faster_parse_ins(input);
    let mut values = vec![0u64; ins.len()];
    let mut index = ins.len() - 1;
    let mut a = 0u64;
    loop {
        let a_t = a + values[index];
        // println!("{values:?} : {index},{a}:{a_t}");
        if step_fast(Registers::new(a_t, 0, 0), &ins) == ins[index] {
            if index == 0 {
                return a_t;
            } else {
                index -= 1;
                a = a_t << 3;
            }
        } else {
            values[index] += 1;
            while values[index] == 8 {
                values[index] = 0;
                index += 1;
                a >>= 3;
                a &= !0b111;

                values[index] += 1;
                // a += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::day17::solver_part2_backwrds;

    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    static INPUT2: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), "4,6,3,5,6,3,5,2,1,0")
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT2)), 117440)
    }

    #[test]
    fn part2_2() {
        assert_eq!(solver_part2_backwrds(&input_generator(INPUT2)), 117440)
    }
}

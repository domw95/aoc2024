use std::{fs::File, io::Read};

use aoc2024;
fn main() {
    let mut input = String::new();
    let file = File::open("input/2024/day20.txt")
        .unwrap()
        .read_to_string(&mut input);
    let input = &aoc2024::day20::input_generator(&input);
    for _ in 0..1000 {
        aoc2024::day20::solver_part2_slice(input);
    }
}

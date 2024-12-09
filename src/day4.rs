use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use aoc_tools::grid::Grid;
use itertools::Itertools;

type Input = String;

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

fn parse_char(input: &str) -> Grid<char> {
    let width = input.lines().next().unwrap().len();
    Grid::from_iter(&mut input.lines().flat_map(|l| l.chars()), width)
}

fn parse_byte(input: &str) -> Grid<u8> {
    let width = input.lines().next().unwrap().len();
    Grid::from_iter(&mut input.lines().flat_map(|l| l.bytes()), width)
}

#[aoc(day4, part1)]
fn solver_part1(input: &Input) -> usize {
    let grid = parse_char(input);
    grid.coord_iter()
        .flat_map(|coord| {
            grid.all_direction_iterators(coord)
                .into_iter()
                .map(|iter| iter.take(4).map(|(c, _)| *c).collect::<String>() == "XMAS")
                .filter(|v| *v)
                .collect::<Vec<bool>>()
        })
        .count()
}

#[aoc(day4, part1, FAST)]
fn solver_part1_fast(input: &Input) -> usize {
    let grid = parse_char(input);
    grid.coord_iter()
        .flat_map(|coord| {
            grid.all_direction_iterators(coord)
                .into_iter()
                .map(|mut iter| {
                    matches!(iter.next(), Some(('X', _)))
                        && matches!(iter.next(), Some(('M', _)))
                        && matches!(iter.next(), Some(('A', _)))
                        && matches!(iter.next(), Some(('S', _)))
                })
                .filter(|v| *v)
                .collect::<Vec<bool>>()
        })
        .count()
}

#[aoc(day4, part1, FAST_FLATTEN)]
fn solver_part1_fast_flatten(input: &Input) -> usize {
    let grid = parse_char(input);
    grid.coord_iter()
        .flat_map(|coord| {
            grid.all_direction_iterators(coord)
                .into_iter()
                .filter_map(|mut iter| {
                    if matches!(iter.next(), Some(('X', _)))
                        && matches!(iter.next(), Some(('M', _)))
                        && matches!(iter.next(), Some(('A', _)))
                        && matches!(iter.next(), Some(('S', _)))
                    {
                        Some(())
                    } else {
                        None
                    }
                })
        })
        .count()
}

#[aoc(day4, part1, FAST_FLATTEN_BYTES)]
fn solver_part1_fast_flatten_bytes(input: &Input) -> usize {
    let grid = parse_byte(input);
    grid.coord_iter()
        .flat_map(|coord| {
            grid.all_direction_iterators(coord)
                .into_iter()
                .filter_map(|mut iter| {
                    if matches!(iter.next(), Some((b'X', _)))
                        && matches!(iter.next(), Some((b'M', _)))
                        && matches!(iter.next(), Some((b'A', _)))
                        && matches!(iter.next(), Some((b'S', _)))
                    {
                        Some(())
                    } else {
                        None
                    }
                })
        })
        .count()
}

#[aoc(day4, part1, FAST_FLATTEN_BYTES_FILTER)]
fn solver_part1_fast_flatten_bytes_filter(input: &Input) -> usize {
    let grid = parse_byte(input);
    grid.iter()
        .filter(|(_, &b)| b == b'X')
        .flat_map(|(coord, _b)| {
            grid.all_direction_iterators(coord)
                .into_iter()
                .filter_map(|mut iter| {
                    iter.next();
                    if matches!(iter.next(), Some((b'M', _)))
                        && matches!(iter.next(), Some((b'A', _)))
                        && matches!(iter.next(), Some((b'S', _)))
                    {
                        Some(())
                    } else {
                        None
                    }
                })
        })
        .count()
}

#[aoc(day4, part2)]
fn solver_part2(input: &Input) -> usize {
    let grid = parse_char(input);
    grid.coord_iter()
        .filter(|coord| {
            if grid[*coord] == 'A' {
                let mut diags = grid
                    .diags(coord)
                    .into_iter()
                    .flatten()
                    .copied()
                    .collect_vec();
                (0..4).any(|_| {
                    if diags == vec!['M', 'M', 'S', 'S'] {
                        true
                    } else {
                        diags.rotate_left(1);
                        false
                    }
                })
            } else {
                false
            }
        })
        .count()
}

#[aoc(day4, part2, ITER)]
fn solver_part2_iter(input: &Input) -> usize {
    let target = vec![b'M', b'M', b'S', b'S'];
    let grid = parse_byte(input);
    grid.iter()
        .filter(|(coord, b)| {
            if **b == b'A' {
                let mut diags = grid
                    .diags(coord)
                    .into_iter()
                    .flatten()
                    .copied()
                    .collect_vec();

                diags == target
                    || (0..3).any(|_| {
                        diags.rotate_right(1);
                        diags == target
                    })
            } else {
                false
            }
        })
        .count()
}

#[aoc(day4, part2, ITER_ARRAY)]
fn solver_part2_iter_array(input: &Input) -> usize {
    let targets = [
        [b'M', b'M', b'S', b'S'],
        [b'M', b'S', b'S', b'M'],
        [b'S', b'S', b'M', b'M'],
        [b'S', b'M', b'M', b'S'],
    ];
    let grid = parse_byte(input);
    grid.iter()
        .filter(|(coord, b)| {
            if **b == b'A' {
                let diag = grid.diags(coord).map(|d| *d.unwrap_or(&0));
                if !diag.iter().all(|&d| d != 0) {
                    false
                } else {
                    targets.iter().any(|&target| target == diag)
                }
            } else {
                false
            }
        })
        .count()
}

#[aoc(day4, part2, ITER_INNER)]
fn solver_part2_iter_inner(input: &Input) -> usize {
    let grid = parse_byte(input);
    grid.iter()
        .filter(|(coord, b)| {
            if **b == b'A'
                && coord.x != 0
                && coord.y != 0
                && coord.x as usize != grid.width - 1
                && coord.y as usize != grid.height - 1
            {
                let mut diags = grid
                    .diags_unchecked(coord)
                    .into_iter()
                    .copied()
                    .collect_vec();
                let target = vec![b'M', b'M', b'S', b'S'];
                diags == target
                    || (0..3).any(|_| {
                        diags.rotate_right(1);
                        diags == target
                    })
            } else {
                false
            }
        })
        .count()
}

#[aoc(day4, part2, ITER_INNER_ARRAY)]
fn solver_part2_iter_inner_array(input: &Input) -> usize {
    let targets = [
        [b'M', b'M', b'S', b'S'],
        [b'M', b'S', b'S', b'M'],
        [b'S', b'S', b'M', b'M'],
        [b'S', b'M', b'M', b'S'],
    ];
    let grid = parse_byte(input);
    grid.iter()
        .filter(|(coord, b)| {
            if **b == b'A'
                && coord.x != 0
                && coord.y != 0
                && coord.x as usize != grid.width - 1
                && coord.y as usize != grid.height - 1
            {
                let diags = grid.diags_unchecked(coord).map(|c| *c);

                targets.iter().any(|target| *target == diags)
            } else {
                false
            }
        })
        .count()
}
#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 18)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 9)
    }
}

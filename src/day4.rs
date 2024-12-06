use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use aoc_tools::grid::Grid;
use itertools::Itertools;

type Input = Grid<char>;

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Input {
    let width = input.lines().next().unwrap().len();
    Grid::from_iter(&mut input.lines().flat_map(|l| l.chars()), width)
}

#[aoc(day4, part1)]
fn solver_part1(grid: &Input) -> usize {
    grid.coord_iter()
        .flat_map(|coord| {
            grid.all_direction_iterators(coord)
                .into_iter()
                .map(|iter| iter.take(4).collect::<String>() == "XMAS")
                .filter(|v| *v)
                .collect::<Vec<bool>>()
        })
        .count()
}

#[aoc(day4, part1, FAST)]
fn solver_part1_fast(grid: &Input) -> usize {
    grid.coord_iter()
        .flat_map(|coord| {
            grid.all_direction_iterators(coord)
                .into_iter()
                .map(|mut iter| {
                    matches!(iter.next(), Some('X'))
                        && matches!(iter.next(), Some('M'))
                        && matches!(iter.next(), Some('A'))
                        && matches!(iter.next(), Some('S'))
                })
                .filter(|v| *v)
                .collect::<Vec<bool>>()
        })
        .count()
}

#[aoc(day4, part2)]
fn solver_part2(grid: &Input) -> usize {
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

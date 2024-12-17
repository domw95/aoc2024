use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use aoc_tools::grid::Coord;
use aoc_tools::grid::Grid;
use itertools::Itertools;

type Input = String;

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

fn parse_input(input: &str) -> (Grid<u8>, std::str::Lines<'_>) {
    let width = input.lines().next().unwrap().len();
    let mut lines = input.lines();
    let grid = Grid::from_iter(
        &mut lines
            .take_while_ref(|l| !l.is_empty())
            .flat_map(|l| l.bytes()),
        width,
    );
    lines.next();
    (grid, lines)
}

fn parse_input_2(input: &str) -> (Grid<u8>, std::str::Lines<'_>) {
    let width = input.lines().next().unwrap().len() * 2;
    let mut lines = input.lines();
    let grid = Grid::from_iter(
        &mut lines.take_while_ref(|l| !l.is_empty()).flat_map(|l| {
            l.bytes().flat_map(|b| match b {
                b'#' => [b'#', b'#'],
                b'.' => [b'.', b'.'],
                b'@' => [b'@', b'.'],
                _ => [b'[', b']'],
            })
        }),
        width,
    );
    lines.next();
    (grid, lines)
}

#[aoc(day15, part1)]
fn solver_part1(input: &Input) -> usize {
    let (mut grid, lines) = parse_input(input);
    // find start pos
    let mut bot = Coord::new(0, 0);
    for (c, i) in grid.iter() {
        if *i == b'@' {
            bot = c;
            break;
        }
    }
    grid[bot] = b'.';

    // iter instructions
    for ins in lines.flat_map(|l| l.bytes()) {
        match ins {
            b'>' => {
                let dest = bot.east();
                match grid[dest] {
                    b'.' => bot = dest,
                    b'O' => {
                        let mut next = dest.east();
                        loop {
                            match grid[next] {
                                b'.' => {
                                    grid[next] = b'O';
                                    grid[dest] = b'.';
                                    bot = dest;
                                    break;
                                }
                                b'O' => next = next.east(),
                                _ => break,
                            }
                        }
                    }
                    _ => (),
                }
            }
            b'v' => {
                let dest = bot.south();
                match grid[dest] {
                    b'.' => bot = dest,
                    b'O' => {
                        let mut next = dest.south();
                        loop {
                            match grid[next] {
                                b'.' => {
                                    grid[next] = b'O';
                                    grid[dest] = b'.';
                                    bot = dest;
                                    break;
                                }
                                b'O' => next = next.south(),
                                _ => break,
                            }
                        }
                    }
                    _ => (),
                }
            }
            b'<' => {
                let dest = bot.west();
                match grid[dest] {
                    b'.' => bot = dest,
                    b'O' => {
                        let mut next = dest.west();
                        loop {
                            match grid[next] {
                                b'.' => {
                                    grid[next] = b'O';
                                    grid[dest] = b'.';
                                    bot = dest;
                                    break;
                                }
                                b'O' => next = next.west(),
                                _ => break,
                            }
                        }
                    }
                    _ => (),
                }
            }
            b'^' => {
                let dest = bot.north();
                match grid[dest] {
                    b'.' => bot = dest,
                    b'O' => {
                        let mut next = dest.north();
                        loop {
                            match grid[next] {
                                b'.' => {
                                    grid[next] = b'O';
                                    grid[dest] = b'.';
                                    bot = dest;
                                    break;
                                }
                                b'O' => next = next.north(),
                                _ => break,
                            }
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }

    // Sum box positions
    grid.iter()
        .filter_map(|(c, i)| {
            if *i == b'O' {
                Some(c.x as usize + 100 * c.y as usize)
            } else {
                None
            }
        })
        .sum()
}

#[aoc(day15, part2)]
fn solver_part2(input: &Input) -> usize {
    let (mut grid, lines) = parse_input_2(input);
    // find start pos
    let mut bot = Coord::new(0, 0);
    for (c, i) in grid.iter() {
        if *i == b'@' {
            bot = c;
            break;
        }
    }
    grid[bot] = b'.';

    // iter instructions
    for ins in lines.flat_map(|l| l.bytes()) {
        match ins {
            b'>' => {
                let dest = bot.east();
                match grid[dest] {
                    b'.' => bot = dest,
                    b'[' => {
                        let mut next = dest.east();
                        loop {
                            match grid[next] {
                                b'.' => {
                                    grid[next] = b'O';
                                    grid[dest] = b'.';
                                    bot = dest;
                                    break;
                                }
                                b'[' | b']' => next = next.east(),
                                _ => break,
                            }
                        }
                    }
                    _ => (),
                }
            }
            b'<' => {
                let dest = bot.west();
                match grid[dest] {
                    b'.' => bot = dest,
                    b']' => {
                        let mut next = dest.west();
                        loop {
                            match grid[next] {
                                b'.' => {
                                    grid[next] = b'O';
                                    grid[dest] = b'.';
                                    bot = dest;
                                    break;
                                }
                                b'[' | b']' => next = next.west(),
                                _ => break,
                            }
                        }
                    }
                    _ => (),
                }
            }
            b'v' => {
                let dest = bot.south();
                match grid[dest] {
                    b'.' => bot = dest,
                    b'O' => {
                        let mut next = dest.south();
                        loop {
                            match grid[next] {
                                b'.' => {
                                    grid[next] = b'O';
                                    grid[dest] = b'.';
                                    bot = dest;
                                    break;
                                }
                                b'O' => next = next.south(),
                                _ => break,
                            }
                        }
                    }
                    _ => (),
                }
            }

            b'^' => {
                let dest = bot.north();
                match grid[dest] {
                    b'.' => bot = dest,
                    b'O' => {
                        let mut next = dest.north();
                        loop {
                            match grid[next] {
                                b'.' => {
                                    grid[next] = b'O';
                                    grid[dest] = b'.';
                                    bot = dest;
                                    break;
                                }
                                b'O' => next = next.north(),
                                _ => break,
                            }
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    static INPUT2: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 10092)
    }

    #[test]
    fn part1_2() {
        assert_eq!(solver_part1(&input_generator(INPUT2)), 2028)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 0)
    }
}

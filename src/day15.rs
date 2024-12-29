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

fn can_move_up(grid: &Grid<u8>, pos: Coord) -> Option<Vec<Coord>> {
    match grid[pos] {
        b'[' => {
            //
            let left = match grid[pos.north()] {
                b'.' => Some(vec![]),
                b'[' => can_move_up(grid, pos.north()),
                b']' => can_move_up(grid, pos.north_west()),
                _ => None,
            }?;
            let right = match grid[pos.north_east()] {
                b'.' | b']' => Some(vec![]),
                b'[' => can_move_up(grid, pos.north_east()),
                _ => None,
            }?;
            Some(
                [vec![pos, pos.east()], left, right]
                    .into_iter()
                    .flatten()
                    .collect(),
            )
        }

        _ => panic!(),
    }
}

fn can_move_down(grid: &Grid<u8>, pos: Coord) -> Option<Vec<Coord>> {
    match grid[pos] {
        b'[' => {
            //
            let left = match grid[pos.south()] {
                b'.' => Some(vec![]),
                b'[' => can_move_down(grid, pos.south()),
                b']' => can_move_down(grid, pos.south_west()),
                _ => None,
            }?;
            let right = match grid[pos.south_east()] {
                b'.' | b']' => Some(vec![]),
                b'[' => can_move_down(grid, pos.south_east()),
                _ => None,
            }?;
            Some(
                [vec![pos, pos.east()], left, right]
                    .into_iter()
                    .flatten()
                    .collect(),
            )
        }

        _ => panic!(),
    }
}

fn move_up(grid: &mut Grid<u8>, mut vec: Vec<Coord>) {
    vec.sort_by_key(|c| c.y);
    for c in vec.into_iter().unique() {
        grid[c.north()] = grid[c];
        grid[c] = b'.';
    }
}

fn move_down(grid: &mut Grid<u8>, mut vec: Vec<Coord>) {
    vec.sort_by_key(|c| c.y);
    vec.reverse();
    for c in vec.into_iter().unique() {
        grid[c.south()] = grid[c];
        grid[c] = b'.';
    }
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
    // grid.print_func(|&b| format!("{}", b as char));
    // println!();
    // iter instructions
    for ins in lines.flat_map(|l| l.bytes()) {
        // grid.print_func(|&b| format!("{}", b as char));
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
                                    grid[dest] = b'.';
                                    grid[next] = b']';
                                    next = next.west();
                                    while next != dest {
                                        if grid[next] == b']' {
                                            grid[next] = b'[';
                                        } else {
                                            grid[next] = b']';
                                        }
                                        next = next.west();
                                    }

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
                                    grid[dest] = b'.';
                                    grid[next] = b'[';
                                    next = next.east();
                                    while next != dest {
                                        if grid[next] == b']' {
                                            grid[next] = b'[';
                                        } else {
                                            grid[next] = b']';
                                        }
                                        next = next.east();
                                    }
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
            b'^' => {
                let dest = bot.north();
                match grid[dest] {
                    b'.' => bot = dest,
                    b'[' => {
                        if let Some(vec) = can_move_up(&grid, dest) {
                            // dbg!(&vec);
                            move_up(&mut grid, vec);

                            bot = dest;
                        }
                    }
                    b']' => {
                        if let Some(vec) = can_move_up(&grid, dest.west()) {
                            // dbg!(&vec);
                            move_up(&mut grid, vec);
                            bot = dest;
                        }
                    }
                    _ => (),
                }
            }
            b'v' => {
                let dest = bot.south();
                match grid[dest] {
                    b'.' => bot = dest,
                    b'[' => {
                        if let Some(vec) = can_move_down(&grid, dest) {
                            // dbg!(&vec);
                            move_down(&mut grid, vec);
                            bot = dest;
                        }
                    }
                    b']' => {
                        if let Some(vec) = can_move_down(&grid, dest.west()) {
                            // dbg!(&vec);
                            move_down(&mut grid, vec);
                            bot = dest;
                        }
                    }
                    _ => (),
                }
            }
            _ => (),
        }
    }
    // grid.print_func(|&b| format!("{}", b as char));
    // Sum box positions
    grid.iter()
        .filter_map(|(c, i)| {
            if *i == b'[' {
                Some(c.x as usize + 100 * c.y as usize)
            } else {
                None
            }
        })
        .sum()
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

    static INPUT3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    static INPUT4: &str = "#######
#.....#
#.O.O@#
#..O..#
#..O..#
#.....#
#######

<v<<>vv<^^";

    static INPUT5: &str = "#######
#.....#
#.OO@.#
#.....#
#######

<<";
    static INPUT6: &str = "#######
#.....#
#.O#..#
#..O@.#
#.....#
#######

<v<<^";

    static INPUT7: &str = "#######
#.....#
#.#O..#
#..O@.#
#.....#
#######

<v<^";

    static INPUT8: &str = "######
#....#
#.O..#
#.OO@#
#.O..#
#....#
######

<vv<<^";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 10092)
    }

    #[test]
    fn part1_2() {
        assert_eq!(solver_part1(&input_generator(INPUT2)), 2028)
    }

    #[test]
    fn part2_1() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 9021)
    }

    #[test]
    fn part2_2() {
        assert_eq!(solver_part2(&input_generator(INPUT3)), 618)
    }

    #[test]
    fn part2_4() {
        assert_eq!(solver_part2(&input_generator(INPUT4)), 822)
    }

    #[test]
    fn part2_5() {
        assert_eq!(solver_part2(&input_generator(INPUT5)), 406)
    }

    #[test]
    fn part2_6() {
        assert_eq!(solver_part2(&input_generator(INPUT6)), 509)
    }

    #[test]
    fn part2_7() {
        assert_eq!(solver_part2(&input_generator(INPUT7)), 511)
    }

    #[test]
    fn part2_8() {
        assert_eq!(solver_part2(&input_generator(INPUT8)), 816)
    }
}

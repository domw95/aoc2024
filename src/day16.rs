use aoc_runner_derive::aoc;
use aoc_runner_derive::aoc_generator;
use aoc_tools::grid::Coord;
use aoc_tools::grid::Grid;
use fxhash::FxHashMap;
use fxhash::FxHashSet;

type Input = String;

#[aoc_generator(day16)]
fn input_generator(input: &str) -> Input {
    input.to_string()
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum Orientation {
    North,
    East,
    South,
    West,
}

impl Orientation {
    fn as_index(&self) -> usize {
        *self as usize
    }

    fn adjacent(&self) -> [Self; 2] {
        match self {
            Orientation::North => [Orientation::East, Orientation::West],
            Orientation::East => [Orientation::North, Orientation::South],
            Orientation::South => [Orientation::East, Orientation::West],
            Orientation::West => [Orientation::North, Orientation::South],
        }
    }
}

impl Orientation {
    fn _cost(&self, other: &Self) -> usize {
        match (self, other) {
            (Orientation::North, Orientation::North) => 0,
            (Orientation::North, Orientation::East) => 1000,
            (Orientation::North, Orientation::South) => 2000,
            (Orientation::North, Orientation::West) => 1000,
            (Orientation::East, Orientation::North) => 1000,
            (Orientation::East, Orientation::East) => 0,
            (Orientation::East, Orientation::South) => 1000,
            (Orientation::East, Orientation::West) => 2000,
            (Orientation::South, Orientation::North) => 2000,
            (Orientation::South, Orientation::East) => 1000,
            (Orientation::South, Orientation::South) => 0,
            (Orientation::South, Orientation::West) => 1000,
            (Orientation::West, Orientation::North) => 1000,
            (Orientation::West, Orientation::East) => 2000,
            (Orientation::West, Orientation::South) => 1000,
            (Orientation::West, Orientation::West) => 0,
        }
    }
}

fn forwards(pos: &Coord, direction: &Orientation) -> Coord {
    match direction {
        Orientation::North => pos.north(),
        Orientation::East => pos.east(),
        Orientation::South => pos.south(),
        Orientation::West => pos.west(),
    }
}

#[aoc(day16, part1)]
fn solver_part1(input: &Input) -> u32 {
    let width = input.lines().next().unwrap().len();
    let grid = Grid::from_iter(&mut input.lines().flat_map(|l| l.bytes()), width);
    let mut distances: [_; 4] =
        core::array::from_fn(|_| Grid::new(u32::MAX, grid.width, grid.height));

    let mut pos = Coord::new(0, 0);
    for (c, b) in grid.iter() {
        if *b == b'S' {
            pos = c;
            break;
        }
    }
    let mut direction = Orientation::East;
    let mut distance = 0;
    let mut unvisted = FxHashSet::default();

    distances[direction.as_index()][pos] = 0;
    loop {
        let adj = direction.adjacent();
        for a in adj {
            if distances[a.as_index()][pos] > distance + 1000 {
                distances[a.as_index()][pos] = distance + 1000;
                unvisted.insert((pos, a));
            }
        }
        let forward = forwards(&pos, &direction);
        if let Some(&b) = grid.checked_index(&forward) {
            // dbg!(&forward, b);
            if b == b'.' || b == b'E' {
                let d = distances[direction.as_index()][forward];
                if d > distance + 1 {
                    distances[direction.as_index()][forward] = distance + 1;
                    unvisted.insert((forward, direction));
                }
            }
        }

        // dbg!(&unvisted);
        // find minimum
        if let Some(min) = unvisted
            .iter()
            .map(|(pos, d)| ((pos, d), distances[d.as_index()][*pos]))
            .min_by_key(|c| c.1)
        {
            pos = *min.0 .0;
            direction = *min.0 .1;
            distance = min.1;
            unvisted.remove(&(pos, direction));
        } else {
            break;
        }
    }
    for (c, b) in grid.iter() {
        if *b == b'E' {
            pos = c;
            break;
        }
    }
    distances.iter().map(|g| g[pos]).min().unwrap()
}

#[aoc(day16, part1, MAP)]
fn solver_part1_map(input: &Input) -> u32 {
    let width = input.lines().next().unwrap().len();
    let grid = Grid::from_iter(&mut input.lines().flat_map(|l| l.bytes()), width);
    let mut distances: [_; 4] =
        core::array::from_fn(|_| Grid::new(u32::MAX, grid.width, grid.height));

    let mut pos = Coord::new(0, 0);
    for (c, b) in grid.iter() {
        if *b == b'S' {
            pos = c;
            break;
        }
    }
    let mut direction = Orientation::East;
    let mut distance = 0;
    let mut unvisted = FxHashMap::default();

    distances[direction.as_index()][pos] = 0;
    loop {
        let adj = direction.adjacent();
        for a in adj {
            if distances[a.as_index()][pos] > distance + 1000 {
                distances[a.as_index()][pos] = distance + 1000;
                unvisted.insert((pos, a), distance + 1000);
            }
        }
        let forward = forwards(&pos, &direction);
        if let Some(&b) = grid.checked_index(&forward) {
            // dbg!(&forward, b);
            if b == b'.' || b == b'E' {
                let d = distances[direction.as_index()][forward];
                if d > distance + 1 {
                    distances[direction.as_index()][forward] = distance + 1;
                    unvisted.insert((forward, direction), distance + 1);
                }
            }
        }

        // dbg!(&unvisted);
        // find minimum
        if let Some(min) = unvisted.iter().min_by_key(|c| c.1) {
            pos = min.0 .0;
            direction = min.0 .1;
            distance = *min.1;
            unvisted.remove(&(pos, direction));
        } else {
            break;
        }
    }
    for (c, b) in grid.iter() {
        if *b == b'E' {
            pos = c;
            break;
        }
    }
    distances.iter().map(|g| g[pos]).min().unwrap()
}

#[aoc(day16, part2)]
fn solver_part2(_input: &Input) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use crate::day16::solver_part1_map;

    use super::input_generator;
    use super::solver_part1;
    use super::solver_part2;

    static INPUT: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";
    static INPUT2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    static INPUT3: &str = "###########################
#######################..E#
######################..#.#
#####################..##.#
####################..###.#
###################..##...#
##################..###.###
#################..####...#
################..#######.#
###############..##.......#
##############..###.#######
#############..####.......#
############..###########.#
###########..##...........#
##########..###.###########
#########..####...........#
########..###############.#
#######..##...............#
######..###.###############
#####..####...............#
####..###################.#
###..##...................#
##..###.###################
#..####...................#
#.#######################.#
#S........................#
###########################";
    static INPUT4: &str = "##########
#.......E#
#.##.#####
#..#.....#
##.#####.#
#S.......#
##########";

    static INPUT5: &str = "########################################################
#.........#.........#.........#.........#.........#...E#
#.........#.........#.........#.........#.........#....#
#....#....#....#....#....#....#....#....#....#....#....#
#....#....#....#....#....#....#....#....#....#....#....#
#....#....#....#....#....#....#....#....#....#....#....#
#....#....#....#....#....#....#....#....#....#....#....#
#....#.........#.........#.........#.........#.........#
#S...#.........#.........#.........#.........#.........#
########################################################";

    static INPUT6: &str = "..E
...
...
S..";

    #[test]
    fn part1() {
        assert_eq!(solver_part1(&input_generator(INPUT)), 7036)
    }

    #[test]
    fn part1_map() {
        assert_eq!(solver_part1_map(&input_generator(INPUT)), 7036)
    }

    #[test]
    fn part1_2() {
        assert_eq!(solver_part1(&input_generator(INPUT2)), 11048)
    }

    #[test]
    fn part1_3() {
        assert_eq!(solver_part1(&input_generator(INPUT3)), 21148)
    }

    #[test]
    fn part1_4() {
        assert_eq!(solver_part1(&input_generator(INPUT4)), 4013)
    }

    #[test]
    fn part1_5() {
        assert_eq!(solver_part1(&input_generator(INPUT5)), 21110)
    }

    #[test]
    fn part1_6() {
        assert_eq!(solver_part1(&input_generator(INPUT6)), 1005)
    }

    #[test]
    fn part2() {
        assert_eq!(solver_part2(&input_generator(INPUT)), 0)
    }
}

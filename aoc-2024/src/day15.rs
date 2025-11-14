use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::branch::alt;
use nom::character::complete::{char, newline, one_of};
use nom::combinator::{map, opt};
use nom::multi::{fold_many1, many1};
use nom::sequence::terminated;
use std::ops::{Index, IndexMut};

#[derive(Debug, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

type Moves = Vec<Direction>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    row: i32,
    col: i32,
}

type Coordinates = Vec<Coordinate>;

impl Coordinate {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
    fn neighbor(&self, direction: &Direction) -> Self {
        match *direction {
            Direction::Up => Self::new(self.row - 1, self.col),
            Direction::Down => Self::new(self.row + 1, self.col),
            Direction::Left => Self::new(self.row, self.col - 1),
            Direction::Right => Self::new(self.row, self.col + 1),
        }
    }
    fn gps(&self) -> i32 {
        100 * self.row + self.col
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Side {
    Both,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Box(Side),
    Robot,
}

struct Robot {
    location: Coordinate,
}

impl Robot {
    fn new(location: Coordinate) -> Robot {
        Robot { location }
    }
}

struct Arena {
    width: usize,
    height: usize,
    robot: Robot,
    tiles: Vec<Vec<Tile>>,
}

impl Arena {
    fn new(tiles: Vec<Vec<Tile>>) -> Arena {
        let height = tiles.len();
        let width = tiles.first().map_or(0, |row| row.len());
        assert!(tiles.iter().all(|row| row.len() == width));

        // Find the robot location
        let mut robot_coordinates = vec![];
        for (row_index, row) in tiles.iter().enumerate() {
            for (col_index, tile) in row.iter().enumerate() {
                if tile == &Tile::Robot {
                    let coord = Coordinate::new(row_index as i32, col_index as i32);
                    robot_coordinates.push(coord);
                }
            }
        }

        assert_eq!(robot_coordinates.len(), 1);
        let robot_coord = robot_coordinates
            .first()
            .expect("A single robot must exist");

        let robot = Robot::new(*robot_coord);
        Arena {
            width,
            height,
            robot,
            tiles,
        }
    }
    fn show(&self) {
        for row in &self.tiles {
            for tile in row {
                match tile {
                    Tile::Wall => print!("#"),
                    Tile::Empty => print!("."),
                    Tile::Box(side) => match side {
                        Side::Both => print!("O"),
                        Side::Left => print!("["),
                        Side::Right => print!("]"),
                    },
                    Tile::Robot => print!("@"),
                }
                print!(" ")
            }
            println!();
        }
    }
    fn make_double_wide(&mut self) {
        self.width *= 2;
        self.robot.location.col *= 2;
        for row in &mut self.tiles {
            *row = row
                .iter()
                .flat_map(|tile| match tile {
                    Tile::Wall => [Tile::Wall, Tile::Wall],
                    Tile::Empty => [Tile::Empty, Tile::Empty],
                    Tile::Box(Side::Both) => [Tile::Box(Side::Left), Tile::Box(Side::Right)],
                    Tile::Box(Side::Left) => panic!("Double-wide boxes can't double again"),
                    Tile::Box(Side::Right) => panic!("Double-wide boxes can't double again"),
                    Tile::Robot => [Tile::Robot, Tile::Empty],
                })
                .collect();
        }
    }
    fn apply_move(
        &mut self,
        direction: &Direction,
        coord: Option<Coordinate>,
        apply: bool,
    ) -> bool {
        let curr_loc = coord.unwrap_or(self.robot.location);

        let next_loc = curr_loc.neighbor(direction);
        assert!(self.coordinate_is_valid(&next_loc));

        // println!("Moving from {:?} ({:?}) to {:?} ({:?})", curr_loc, self[&curr_loc], next_loc, self[&next_loc]);

        match (self[&curr_loc], self[&next_loc]) {
            (_, Tile::Wall) => false,
            (tile, Tile::Empty) => {
                let move_ok = true;
                if move_ok && apply {
                    if tile == Tile::Robot {
                        self.robot.location = next_loc;
                    }
                    self[&curr_loc] = Tile::Empty;
                    self[&next_loc] = tile;
                }
                move_ok
            }
            (tile, Tile::Box(Side::Both)) => {
                let move_ok = self.apply_move(direction, Some(next_loc), apply);
                if move_ok && apply {
                    if tile == Tile::Robot {
                        self.robot.location = next_loc;
                    }
                    self[&curr_loc] = Tile::Empty;
                    self[&next_loc] = tile;
                }
                move_ok
            }
            (tile, Tile::Box(side)) => match (direction, side) {
                (_, Side::Both) => panic!("Arena should not be in this state"),
                (Direction::Left | Direction::Right, _) => {
                    let move_ok = self.apply_move(direction, Some(next_loc), apply);
                    if apply {
                        if tile == Tile::Robot {
                            self.robot.location = next_loc;
                        }
                        self[&curr_loc] = Tile::Empty;
                        self[&next_loc] = tile;
                    }
                    move_ok
                }
                (Direction::Up, Side::Left) => {
                    let other_loc = next_loc.neighbor(&Direction::Right);
                    let next_ok = self.apply_move(&Direction::Up, Some(next_loc), apply);
                    let other_ok = self.apply_move(&Direction::Up, Some(other_loc), apply);
                    if next_ok && other_ok && apply {
                        if tile == Tile::Robot {
                            self.robot.location = next_loc;
                        }
                        self[&curr_loc] = Tile::Empty;
                        self[&other_loc] = Tile::Empty;
                        self[&next_loc] = tile;
                    }
                    next_ok && other_ok
                }
                (Direction::Up, Side::Right) => {
                    let other_loc = next_loc.neighbor(&Direction::Left);
                    let next_ok = self.apply_move(&Direction::Up, Some(next_loc), apply);
                    let other_ok = self.apply_move(&Direction::Up, Some(other_loc), apply);
                    if next_ok && other_ok && apply {
                        if tile == Tile::Robot {
                            self.robot.location = next_loc;
                        }
                        self[&curr_loc] = Tile::Empty;
                        self[&other_loc] = Tile::Empty;
                        self[&next_loc] = tile;
                    }
                    next_ok && other_ok
                }
                (Direction::Down, Side::Left) => {
                    let other_loc = next_loc.neighbor(&Direction::Right);
                    let next_ok = self.apply_move(&Direction::Down, Some(next_loc), apply);
                    let other_ok = self.apply_move(&Direction::Down, Some(other_loc), apply);
                    if next_ok && other_ok && apply {
                        if tile == Tile::Robot {
                            self.robot.location = next_loc;
                        }
                        self[&curr_loc] = Tile::Empty;
                        self[&other_loc] = Tile::Empty;
                        self[&next_loc] = tile;
                    }
                    next_ok && other_ok
                }
                (Direction::Down, Side::Right) => {
                    let other_loc = next_loc.neighbor(&Direction::Left);
                    let next_ok = self.apply_move(&Direction::Down, Some(next_loc), apply);
                    let other_ok = self.apply_move(&Direction::Down, Some(other_loc), apply);
                    if next_ok && other_ok && apply {
                        if tile == Tile::Robot {
                            self.robot.location = next_loc;
                        }
                        self[&curr_loc] = Tile::Empty;
                        self[&other_loc] = Tile::Empty;
                        self[&next_loc] = tile;
                    }
                    next_ok && other_ok
                }
            },
            (_, Tile::Robot) => panic!("Robot should not be in this tile"),
        }
    }
    fn apply_moves(&mut self, moves: &Moves) {
        for direction in moves {
            if self.apply_move(direction, None, false) {
                self.apply_move(direction, None, true);
            }
            // self.show();
        }
    }
    fn coordinate_is_valid(&self, c: &Coordinate) -> bool {
        c.row >= 0 && c.row < self.height as i32 && c.col >= 0 && c.col < self.width as i32
    }
    fn coordinates(&self) -> CoordinateIterator {
        CoordinateIterator {
            current_index: 0,
            rows: self.height,
            cols: self.width,
        }
    }
}

impl Index<&Coordinate> for Arena {
    type Output = Tile;

    fn index(&self, c: &Coordinate) -> &Self::Output {
        assert!(self.coordinate_is_valid(c));
        &self.tiles[c.row as usize][c.col as usize]
    }
}

impl IndexMut<&Coordinate> for Arena {
    fn index_mut(&mut self, c: &Coordinate) -> &mut Self::Output {
        assert!(self.coordinate_is_valid(c));
        &mut self.tiles[c.row as usize][c.col as usize]
    }
}

struct CoordinateIterator {
    current_index: usize,
    rows: usize,
    cols: usize,
}

impl Iterator for CoordinateIterator {
    type Item = Coordinate;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.rows * self.cols {
            let row = (self.current_index / self.cols) as i32;
            let col = (self.current_index % self.cols) as i32;
            self.current_index += 1;
            Some(Coordinate::new(row, col))
        } else {
            None
        }
    }
}

fn parse_arena(i: &str) -> IResult<&str, Arena> {
    let (i, tiles) = many1(terminated(
        many1(alt((
            map(char('#'), |_| Tile::Wall),
            map(char('.'), |_| Tile::Empty),
            map(char('O'), |_| Tile::Box(Side::Both)),
            map(char('@'), |_| Tile::Robot),
        ))),
        opt(newline),
    ))
    .parse(i)?;
    Ok((i, Arena::new(tiles)))
}

fn parse_moves(i: &str) -> IResult<&str, Moves> {
    fold_many1(one_of("<>^v\n"), Moves::new, |mut acc: Moves, c| {
        match c {
            '<' => acc.push(Direction::Left),
            '>' => acc.push(Direction::Right),
            '^' => acc.push(Direction::Up),
            'v' => acc.push(Direction::Down),
            _ => {}
        }
        acc
    })
    .parse(i)
}

fn parse_map_and_moves(i: &str) -> IResult<&str, (Arena, Moves)> {
    let (i, arena) = parse_arena(i)?;
    let (i, moves) = parse_moves(i)?;
    Ok((i, (arena, moves)))
}

fn parse_input(i: &str) -> Result<(Arena, Moves)> {
    parse_map_and_moves(i).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2024
    }

    fn day(&self) -> Day {
        Day::Day15
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let (mut arena, moves) = parse_input(input)?;
        // arena.show();
        arena.apply_moves(&moves);
        let sum: i32 = arena
            .coordinates()
            .filter(|c| arena[c] == Tile::Box(Side::Both))
            .map(|c| c.gps())
            .sum();
        Ok(sum.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let (mut arena, moves) = parse_input(input)?;
        // arena.show();
        arena.make_double_wide();
        arena.apply_moves(&moves);
        let sum: i32 = arena
            .coordinates()
            .filter(|c| arena[c] == Tile::Box(Side::Left))
            .map(|c| c.gps())
            .sum();
        Ok(sum.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use rstest::*;

    #[fixture]
    fn solver() -> Solver {
        Solver {}
    }

    #[rstest]
    fn example_small_part1(solver: Solver) -> Result<()> {
        let input = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
        assert_eq!(solver.solve_part1(input)?, "2028");
        Ok(())
    }

    #[rstest]
    fn example_small_part2(solver: Solver) -> Result<()> {
        let input = "\
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";
        assert_eq!(solver.solve_part2(input)?, "618");
        Ok(())
    }

    #[rstest]
    fn example1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example1)?;
        assert_eq!(solver.solve_part1(&input)?, "10092");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "9021");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "1429911");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "1453087");
        Ok(())
    }
}

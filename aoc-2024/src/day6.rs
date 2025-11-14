use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{newline, one_of};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use std::collections::HashSet;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(&mut self, turn: Turn) {
        *self = self.turned(turn);
    }
    fn turned(&self, turn: Turn) -> Direction {
        match turn {
            Turn::Left => match self {
                Direction::Up => Direction::Left,
                Direction::Left => Direction::Down,
                Direction::Down => Direction::Right,
                Direction::Right => Direction::Up,
            },
            Turn::Right => match self {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            },
        }
    }
}

enum Turn {
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Tile {
    Floor,
    Guard(Direction),
    Block,
    Outside,
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    rows: usize,
    cols: usize,
}

struct Guard {
    direction: Direction,
    position: Coordinate,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Step {
    coordinate: Coordinate,
    direction: Direction,
}

impl Step {
    fn new(coordinate: Coordinate, direction: Direction) -> Step {
        Step {
            coordinate,
            direction,
        }
    }
}

enum StepEvent {
    Walk(Step),
    Stuck,
    Outside,
}

impl Guard {
    fn take_step(&mut self, map: &Map) -> StepEvent {
        for _ in 0..4 {
            let next = self.position.step(self.direction);
            match map.get_tile(&next) {
                Tile::Floor | Tile::Guard(_) => {
                    self.position = next;
                    return StepEvent::Walk(Step::new(next, self.direction));
                }
                Tile::Block => self.direction.turn(Turn::Right),
                Tile::Outside => return StepEvent::Outside,
            }
        }
        StepEvent::Stuck
    }
}

type Steps = Vec<Step>;

#[derive(Debug)]
enum Walk {
    Loop(Steps),
    Exit(Steps),
    Limit(Steps),
}

impl Map {
    fn new(tiles: Vec<Vec<Tile>>) -> Map {
        let rows = tiles.len();
        let cols = tiles.first().map(|row| row.len()).unwrap_or(0);
        assert!(tiles.iter().all(|row| row.len() == cols));
        Map { tiles, rows, cols }
    }
    fn with_new_tile(&self, tile: Tile, coord: &Coordinate) -> Map {
        let mut tiles = self.tiles.clone();
        if self.is_inside(coord) {
            tiles[coord.row as usize][coord.col as usize] = tile;
        } else {
            panic!("Coordinate outside map");
        }
        Map::new(tiles)
    }
    fn get_tile(&self, coord: &Coordinate) -> Tile {
        if self.is_inside(coord) {
            self.tiles[coord.row as usize][coord.col as usize]
        } else {
            Tile::Outside
        }
    }
    fn get_all_coordinates(&self, tile: Tile) -> HashSet<Coordinate> {
        self.tiles
            .iter()
            .enumerate()
            .flat_map(|(row, row_tiles)| {
                row_tiles.iter().enumerate().filter_map(move |(col, t)| {
                    if t == &tile {
                        Some(Coordinate::new(row as i32, col as i32))
                    } else {
                        None
                    }
                })
            })
            .collect()
    }
    fn guard(&self) -> Guard {
        for (row, tiles) in self.tiles.iter().enumerate() {
            for (col, tile) in tiles.iter().enumerate() {
                if let Tile::Guard(direction) = tile {
                    let position = Coordinate::new(row as i32, col as i32);
                    return Guard {
                        direction: *direction,
                        position,
                    };
                }
            }
        }
        panic!("No guard found");
    }
    fn show(&self) {
        for row in &self.tiles {
            for tile in row {
                match tile {
                    Tile::Floor => print!("."),
                    Tile::Guard(Direction::Up) => print!("^"),
                    Tile::Guard(Direction::Down) => print!("v"),
                    Tile::Guard(Direction::Left) => print!("<"),
                    Tile::Guard(Direction::Right) => print!(">"),
                    Tile::Block => print!("#"),
                    Tile::Outside => print!(" "),
                }
                print!(" ");
            }
            println!();
        }
    }
    fn is_inside(&self, coord: &Coordinate) -> bool {
        coord.row >= 0
            && coord.row < self.rows as i32
            && coord.col >= 0
            && coord.col < self.cols as i32
    }
    fn walk(&self, limit: Option<usize>) -> Walk {
        let mut guard = self.guard();
        let mut walk = vec![Step::new(guard.position, guard.direction)];
        let mut walkset = walk.iter().cloned().collect::<HashSet<_>>();
        loop {
            match limit {
                Some(limit) if walk.len() > limit => return Walk::Limit(walk),
                _ => (),
            }
            match guard.take_step(self) {
                StepEvent::Walk(step) => {
                    if walkset.contains(&step) {
                        return Walk::Loop(walk);
                    }
                    walkset.insert(step);
                    walk.push(step);
                }
                StepEvent::Stuck => panic!("Guard stuck"),
                StepEvent::Outside => break,
            }
        }
        Walk::Exit(walk)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    row: i32,
    col: i32,
}

impl Coordinate {
    fn new(row: i32, col: i32) -> Coordinate {
        Coordinate { row, col }
    }
    fn step(&self, direction: Direction) -> Coordinate {
        match direction {
            Direction::Up => Coordinate::new(self.row - 1, self.col),
            Direction::Down => Coordinate::new(self.row + 1, self.col),
            Direction::Left => Coordinate::new(self.row, self.col - 1),
            Direction::Right => Coordinate::new(self.row, self.col + 1),
        }
    }
}

fn parse_map(input: &str) -> IResult<&str, Map> {
    let (input, tiles) = separated_list1(
        newline,
        many1(alt((
            map(tag("."), |_| Tile::Floor),
            map(one_of("^v<>"), |c| match c {
                '^' => Tile::Guard(Direction::Up),
                'v' => Tile::Guard(Direction::Down),
                '<' => Tile::Guard(Direction::Left),
                '>' => Tile::Guard(Direction::Right),
                _ => unreachable!(),
            }),
            map(tag("#"), |_| Tile::Block),
        ))),
    )(input)?;
    let rows = tiles.len();
    let cols = tiles.first().map_or(0, |row| row.len());
    Ok((input, Map { tiles, rows, cols }))
}

fn parse_input(input: &str) -> Result<Map> {
    parse_map(input).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2024
    }

    fn day(&self) -> Day {
        Day::Day6
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let map = parse_input(input)?;
        // map.show();
        match map.walk(None) {
            Walk::Exit(walk) => {
                let result = walk
                    .iter()
                    .map(|s| s.coordinate)
                    .collect::<HashSet<_>>()
                    .len();
                Ok(result.to_string())
            }
            Walk::Loop(_) => Err(AdventError::Other("Loop detected".to_string())),
            Walk::Limit(_) => Err(AdventError::Other("Walk limit reached".to_string())),
        }
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let map = parse_input(input)?;
        let floor_tiles = map.get_all_coordinates(Tile::Floor);
        let result = floor_tiles
            .iter()
            .filter(|coord| {
                let test_map = map.with_new_tile(Tile::Block, coord);
                match test_map.walk(Some(1_000_000)) {
                    Walk::Exit(_) => false,
                    Walk::Loop(_) => true,
                    Walk::Limit(_) => panic!("Walk limit reached"),
                }
            })
            .count();

        Ok(result.to_string())
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
    fn example1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example1)?;
        assert_eq!(solver.solve_part1(&input)?, "41");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "6");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "5145");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "1523");
        Ok(())
    }
}

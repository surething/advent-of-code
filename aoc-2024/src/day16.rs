use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::branch::alt;
use nom::character::complete::{char, newline};
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::sequence::terminated;
use std::cmp::{Ordering, Reverse};
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::ops::Index;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, EnumIter)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn turned(&self, turn: &Turn) -> Self {
        match turn {
            Turn::Left => match self {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
            },
            Turn::Right => match self {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
        }
    }
    fn opposite(&self) -> Self {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::West => Direction::East,
            Direction::East => Direction::West,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Start,
    Exit,
}

impl Tile {
    fn is_wall(&self) -> bool {
        *self == Tile::Wall
    }
    fn is_empty(&self) -> bool {
        *self == Tile::Empty
    }
    fn is_start(&self) -> bool {
        *self == Tile::Start
    }
    fn is_exit(&self) -> bool {
        *self == Tile::Exit
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    row: i32,
    col: i32,
}

impl Coordinate {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
    fn neighbor(&self, direction: &Direction) -> Self {
        match *direction {
            Direction::North => Self::new(self.row - 1, self.col),
            Direction::South => Self::new(self.row + 1, self.col),
            Direction::West => Self::new(self.row, self.col - 1),
            Direction::East => Self::new(self.row, self.col + 1),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Step {
    coordinate: Coordinate,
    direction: Direction,
}

impl Step {
    fn new(coordinate: Coordinate, direction: Direction) -> Self {
        Self {
            coordinate,
            direction,
        }
    }
    fn take(&self, direction: &Direction) -> Self {
        Self::new(self.coordinate.neighbor(direction), *direction)
    }
}

type Paths = Vec<Path>;

#[derive(Debug, Clone)]
struct Path {
    vec: Vec<Step>,
    set: HashSet<Step>,
    score: usize,
}

pub struct ScoredPath(Path);

impl PartialEq for ScoredPath {
    fn eq(&self, other: &Self) -> bool {
        self.0.score == other.0.score
    }
}

impl Eq for ScoredPath {}

impl PartialOrd for ScoredPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.score.cmp(&other.0.score))
    }
}

impl Ord for ScoredPath {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.score.cmp(&other.0.score)
    }
}

impl Path {
    fn new() -> Self {
        Self {
            vec: vec![],
            set: HashSet::new(),
            score: 0,
        }
    }
    fn new_with_step(step: Step) -> Self {
        let mut path = Self::new();
        path.push(step);
        path
    }
    fn last(&self) -> Option<&Step> {
        self.vec.last()
    }
    fn push(&mut self, step: Step) {
        self.vec.push(step);
        self.set.insert(step);
        self.score = self.calculate_score();
    }
    fn pop(&mut self) -> Option<Step> {
        let step = self.vec.pop();
        if let Some(step) = &step {
            self.set.remove(step);
            self.score = self.calculate_score();
        }
        step
    }
    fn contains_coord(&self, coord: &Coordinate) -> bool {
        self.set.iter().any(|s| s.coordinate == *coord)
    }
    fn contains_step(&self, step: &Step) -> bool {
        self.set.contains(step)
    }
    fn reaches_exit(&self, maze: &Maze) -> bool {
        self.vec
            .last()
            .map_or(false, |s| maze[&s.coordinate].is_exit())
    }
    fn calculate_score(&self) -> usize {
        let num_turns = self
            .vec
            .iter()
            .tuple_windows()
            .filter(|(a, b)| a.direction != b.direction)
            .count();
        num_turns * 1000 + self.vec.len() - 1
    }
}

enum Turn {
    Left,
    Right,
}

struct Maze {
    width: usize,
    height: usize,
    tiles: Vec<Vec<Tile>>,
}

impl Maze {
    fn new(tiles: Vec<Vec<Tile>>) -> Self {
        let height = tiles.len();
        let width = tiles.first().map_or(0, |row| row.len());
        assert!(tiles.iter().all(|row| row.len() == width));
        Self {
            width,
            height,
            tiles,
        }
    }
    fn show(&self) {
        for row in &self.tiles {
            for tile in row {
                print!(
                    "{} ",
                    match tile {
                        Tile::Wall => '#',
                        Tile::Empty => '.',
                        Tile::Start => 'S',
                        Tile::Exit => 'E',
                    }
                );
            }
            println!();
        }
    }
    fn show_with_path(&self, path: &Path) {
        for row in 0..self.height {
            for col in 0..self.width {
                let c = Coordinate::new(row as i32, col as i32);
                if path.contains_step(&Step::new(c, Direction::North)) {
                    print!("^ ");
                } else if path.contains_step(&Step::new(c, Direction::South)) {
                    print!("v ");
                } else if path.contains_step(&Step::new(c, Direction::West)) {
                    print!("< ");
                } else if path.contains_step(&Step::new(c, Direction::East)) {
                    print!("> ");
                } else {
                    print!(
                        "{} ",
                        match self[&c] {
                            Tile::Wall => '#',
                            Tile::Empty => '.',
                            Tile::Start => 'S',
                            Tile::Exit => 'E',
                        }
                    );
                }
            }
            println!();
        }
    }
    fn start(&self) -> Option<Step> {
        for c in self.coordinates() {
            if self[&c].is_start() {
                return Some(Step::new(c, Direction::East));
            }
        }
        None
    }
    fn shortest_path(&self, start_step: Step) -> Option<Path> {
        let path = Path::new_with_step(start_step);

        // Do not re-visit the same coordinate.
        let mut visited: HashSet<Coordinate> = HashSet::new();

        // Keep a priority queue - reversed for a minimum heap.
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(ScoredPath(path)));

        // Keep fetching the path with the lowest score.
        while let Some(Reverse(ScoredPath(path))) = heap.pop() {
            if path.reaches_exit(self) {
                return Some(path);
            } else {
                let current_step = path.last().expect("Empty path");
                let next_steps = Direction::iter()
                    .filter(|d| *d != current_step.direction.opposite())
                    .map(|d| current_step.take(&d))
                    .filter(|s| {
                        let next_tile = self[&s.coordinate];
                        next_tile.is_empty() || next_tile.is_exit()
                    })
                    .filter(|s| visited.insert(s.coordinate));

                for next in next_steps {
                    let mut new_path = path.clone();
                    new_path.push(next);
                    heap.push(Reverse(ScoredPath(new_path)));
                }
            }
        }

        None
    }
    fn all_shortest_paths(&self, start_step: Step) -> Paths {
        let mut shortest_paths = Paths::new();
        let mut lowest_score: Option<usize> = None;
        let path = Path::new_with_step(start_step);

        // Keep scores.
        let mut visited: HashMap<Coordinate, usize> = HashMap::new();

        // Keep a priority queue - reversed for a minimum heap.
        let mut heap = BinaryHeap::new();
        heap.push(Reverse(ScoredPath(path)));

        // Keep fetching the path with the lowest score.
        while let Some(Reverse(ScoredPath(path))) = heap.pop() {
            if lowest_score.map(|low| path.score > low).unwrap_or(false) {
                continue;
            } else if path.reaches_exit(self) {
                match lowest_score {
                    Some(low) if path.score < low => {
                        lowest_score = Some(path.score);
                        shortest_paths.clear();
                        shortest_paths.push(path);
                    }
                    Some(low) if path.score == low => {
                        shortest_paths.push(path);
                    }
                    None => {
                        lowest_score = Some(path.score);
                        shortest_paths.push(path);
                    }
                    _ => {}
                }
            } else {
                let current_step = path.last().expect("Empty path");
                let next_steps = Direction::iter()
                    .filter(|d| *d != current_step.direction.opposite())
                    .map(|d| current_step.take(&d))
                    .filter(|s| !self[&s.coordinate].is_wall())
                    .filter(|s| !path.contains_coord(&s.coordinate));

                for next in next_steps {
                    let mut new_path = path.clone();
                    new_path.push(next);

                    let entry = visited.entry(next.coordinate).or_insert(usize::MAX);
                    match new_path.score.cmp(entry) {
                        Ordering::Less => {
                            *entry = new_path.score;
                            heap.push(Reverse(ScoredPath(new_path)));
                        }
                        Ordering::Equal => {
                            heap.push(Reverse(ScoredPath(new_path)));
                        }
                        Ordering::Greater => {
                            if new_path.score < *entry + 2000 {
                                heap.push(Reverse(ScoredPath(new_path)));
                            }
                        }
                    }
                }
            }
        }

        shortest_paths
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

impl Index<&Coordinate> for Maze {
    type Output = Tile;

    fn index(&self, c: &Coordinate) -> &Self::Output {
        assert!(self.coordinate_is_valid(c));
        &self.tiles[c.row as usize][c.col as usize]
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

fn parse_maze(i: &str) -> IResult<&str, Maze> {
    let (i, tiles) = many1(terminated(
        many1(alt((
            map(char('#'), |_| Tile::Wall),
            map(char('.'), |_| Tile::Empty),
            map(char('S'), |_| Tile::Start),
            map(char('E'), |_| Tile::Exit),
        ))),
        opt(newline),
    ))(i)?;
    Ok((i, Maze::new(tiles)))
}

fn parse_input(input: &str) -> Result<Maze> {
    parse_maze(input).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2024
    }

    fn day(&self) -> Day {
        Day::Day16
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let maze = parse_input(input)?;
        // maze.show();
        let start = maze.start().expect("No start found");
        let path = maze.shortest_path(start).expect("No path found");
        Ok(path.score.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let maze = parse_input(input)?;
        let start = maze.start().expect("No start found");
        let all_paths = maze.all_shortest_paths(start);
        let tile_count = all_paths
            .iter()
            .flat_map(|p| p.vec.iter())
            .map(|s| s.coordinate)
            .collect::<HashSet<_>>()
            .len();
        Ok(tile_count.to_string())
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

    #[fixture]
    fn second_example() -> &'static str {
        "\
#################
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
#################"
    }

    #[rstest]
    fn example_second_part_1(solver: Solver, second_example: &str) -> Result<()> {
        assert_eq!(solver.solve_part1(second_example)?, "11048");
        Ok(())
    }

    #[rstest]
    fn example_second_part_2(solver: Solver, second_example: &str) -> Result<()> {
        assert_eq!(solver.solve_part2(second_example)?, "64");
        Ok(())
    }

    #[rstest]
    fn example1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example1)?;
        assert_eq!(solver.solve_part1(&input)?, "7036");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "45");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "130536");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "1024");
        Ok(())
    }
}

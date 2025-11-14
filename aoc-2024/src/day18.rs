use aoc_common::prelude::*;
use aoc_data::prelude::*;
use std::collections::HashSet;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::sequence::{separated_pair, terminated};

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Position {
    x: i64,
    y: i64,
}

impl Position {
    fn move_to(&self, direction: Direction) -> Position {
        match direction {
            Direction::Up => Position { x: self.x, y: self.y - 1 },
            Direction::Down => Position { x: self.x, y: self.y + 1 },
            Direction::Left => Position { x: self.x - 1, y: self.y },
            Direction::Right => Position { x: self.x + 1, y: self.y },
        }
    }
}

struct Path {
    vec: Vec<Position>,
    set: HashSet<Position>,
}

impl Path {
    fn new() -> Self {
        let vec = Vec::new();
        let set = HashSet::new();
        Path { vec, set }
    }
    fn new_with_start(start: Position) -> Self {
        let vec = vec![start];
        let set = HashSet::from([start]);
        Path { vec, set }
    }
    fn last(&self) -> Option<&Position> {
        self.vec.last()
    }
    fn push(&mut self, position: Position) {
        self.vec.push(position);
        self.set.insert(position);
    }
    fn pop(&mut self) -> Option<Position> {
        let result = self.vec.pop();
        if let Some(pos) = &result {
            self.set.remove(pos);
        }
        result
    }
    fn add(&mut self, position: Position) {
        self.vec.push(position);
        self.set.insert(position);
    }
    fn contains(&self, position: Position) -> bool {
        self.set.contains(&position)
    }
    fn len(&self) -> usize {
        self.vec.len()
    }
    fn num_steps(&self) -> usize {
        self.vec.len().max(1) - 1
    }
}

type Positions = Vec<Position>;

#[derive(Debug, Copy, Clone)]
enum Byte {
    Healthy,
    Corrupted,
}

type Bytes = Vec<Byte>;
type Grid = Vec<Bytes>;

struct Memory {
    grid: Grid,
    width: usize,
    height: usize,
}

impl Memory {
    fn new(width: usize, height: usize) -> Self {
        let grid = vec![vec![Byte::Healthy; width]; height];
        Memory { grid, width, height }
    }
    fn start(&self) -> Position {
        Position { x: 0, y: 0 }
    }
    fn exit(&self) -> Position {
        Position { x: self.width as i64 - 1, y: self.height as i64 - 1 }
    }
    fn is_start(&self, position: Position) -> bool {
        self.start().eq(&position)
    }
    fn is_exit(&self, position: Position) -> bool {
        self.exit().eq(&position)
    }
    fn is_valid(&self, position: Position) -> bool {
        position.x >= 0 && position.x < self.width as i64 && position.y >= 0 && position.y < self.height as i64
    }
}

struct CorruptingMemory {
    memory: Memory,
    corruptions: Positions,
}

impl CorruptingMemory {
    fn new(memory: Memory, corruptions: Positions) -> Self {
        CorruptingMemory { memory, corruptions }
    }
    fn shortest_path(&self, start: Position, finish: Position) -> Option<Path> {
        let path = Path::new_with_start(start);
        Some(path)
    }
}

fn parse_position(i: &str) -> IResult<&str, Position> {
    map(
        terminated(
            separated_pair(
                complete::i64,
                tag(","),
                complete::i64,
            ),
            opt(newline)  
        ),
        |(x, y)| Position { x, y }
    )(i)
}

fn parse_positions(i: &str) -> IResult<&str, Positions> {
    many1(parse_position)(i)
}

fn parse_input(input: &str) -> Result<Positions> {
    parse_positions(input).map_and_finish()
} 

enum MemorySize {
    Small,
    Large,
}

struct Solver {
    memory_size: MemorySize,
}

impl Solver {
    fn new(memory_size: MemorySize) -> Self {
        Solver { memory_size }
    }
}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2024
    }

    fn day(&self) -> Day {
        Day::Day18
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        // Take only the first kilobyte of input.
        let corruptions = parse_input(input)?.into_iter().take(1024).collect();

        // Set up the memory arena.
        let memory = match self.memory_size {
            MemorySize::Small => Memory::new(7, 7),
            MemorySize::Large => Memory::new(70, 70),
        };

        // Set up the scenario.
        let start = memory.start();
        let finish = memory.exit();
        let corrupting_memory = CorruptingMemory::new(memory, corruptions);

        // Solve.
        let path = corrupting_memory.shortest_path(start, finish).expect("No path found");
        
        Ok(path.num_steps().to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        Ok("0".to_string())
    }
}

#[cfg(test)]
mod test {

    use super::*;
    use rstest::*;

    #[rstest]
    fn example1() -> Result<()> {
        let solver = Solver::new(MemorySize::Small);
        let input = solver.read_resource(Input::Example1)?;
        assert_eq!(solver.solve_part1(&input)?, "0");
        Ok(())
    }

    #[rstest]
    fn example2() -> Result<()> {
        let solver = Solver::new(MemorySize::Small);
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "0");
        Ok(())
    }

    #[rstest]
    fn part1() -> Result<()> {
        let solver = Solver::new(MemorySize::Large);
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "0");
        Ok(())
    }

    #[rstest]
    fn part2() -> Result<()> {
        let solver = Solver::new(MemorySize::Large);
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "0");
        Ok(())
    }
}

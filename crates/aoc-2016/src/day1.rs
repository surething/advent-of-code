use std::collections::HashSet;
use std::ops::ControlFlow;
use std::ops::ControlFlow::{Break, Continue};
use nom::branch::alt;
use nom::character::complete;
use nom::combinator::opt;
use nom::multi::many1;
use aoc_common::prelude::*;
use aoc_data::prelude::*;

#[derive(Debug, Clone, Copy)]
enum Action {
    Right(u32),
    Left(u32),
}

#[derive(Debug, Clone, Copy)]
enum Turn {
    Right,
    Left,
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

type Position = (i32, i32);

struct State {
    pos: Position,
    dir: Direction,
}

struct TrackedState {
    state: State,
    visited: HashSet<Position>,
}

impl State {
    fn new() -> Self {
        let pos = (0, 0);
        let dir = Direction::North;
        Self { pos, dir }
    }
    fn apply(mut self, action: Action) -> Self {
        match action {
            Action::Right(distance) => self.turn(Turn::Right).move_forward(distance),
            Action::Left(distance) => self.turn(Turn::Left).move_forward(distance),
        }
    }
    fn turn(mut self, turn: Turn) -> Self {
        self.dir = match turn {
            Turn::Right => match self.dir {
                Direction::North => Direction::East,
                Direction::East => Direction::South,
                Direction::South => Direction::West,
                Direction::West => Direction::North,
            },
            Turn::Left => match self.dir {
                Direction::North => Direction::West,
                Direction::West => Direction::South,
                Direction::South => Direction::East,
                Direction::East => Direction::North,
            },
        };
        self
    }
    fn move_forward(mut self, distance: u32) -> Self {
        match self.dir {
            Direction::North => self.pos.1 += distance as i32,
            Direction::East => self.pos.0 += distance as i32,
            Direction::South => self.pos.1 -= distance as i32,
            Direction::West => self.pos.0 -= distance as i32,
        }
        self
    }
    fn norm1(&self) -> u32 {
        self.pos.0.abs() as u32 + self.pos.1.abs() as u32
    }
}

impl TrackedState {
    fn new() -> Self {
        let state = State::new();
        let mut visited = HashSet::new();
        visited.insert(state.pos);
        Self { state, visited }
    }
    fn apply(mut self, action: Action) -> ControlFlow<u32, Self> {
        match action {
            Action::Right(distance) => self.turn(Turn::Right).move_forward(distance),
            Action::Left(distance) => self.turn(Turn::Left).move_forward(distance),
        }
    }
    fn turn(mut self, turn: Turn) -> Self {
        self.state = self.state.turn(turn);
        self
    }
    fn move_forward(mut self, distance: u32) -> ControlFlow<u32, Self> {
        for _ in 0..distance {
            match self.state.dir {
                Direction::North => self.state.pos.1 += 1,
                Direction::East => self.state.pos.0 += 1,
                Direction::South => self.state.pos.1 -= 1,
                Direction::West => self.state.pos.0 -= 1,
            }
            if !self.visited.insert(self.state.pos) {
                return Break(self.state.norm1());
            }
        }
        Continue(self)
    }
}

fn parse_action(i: &str) -> IResult<&str, Action> {
    let (i, turn) = alt((complete::char('R'), complete::char('L'))).parse(i)?;
    let (i, distance) = complete::u32(i)?;
    let action = match turn {
        'R' => Action::Right(distance),
        'L' => Action::Left(distance),
        _ => unreachable!(),
    };
    // Consume optional comma and space.
    let (i, _) = opt((complete::char(','), complete::space0)).parse(i)?;
    Ok((i, action))
}

fn parse_input(i: &str) -> Result<Vec<Action>> {
    many1(parse_action).parse(i).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2016
    }

    fn day(&self) -> Day {
        Day::Day1
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let actions = parse_input(input.trim())?;
        let state = actions.iter().fold(State::new(), |mut state, &action| {
            state.apply(action)
        });
        Ok(state.norm1().to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let actions = parse_input(input.trim())?;
        let mut tracked_state = TrackedState::new();
        for &action in &actions {
            match tracked_state.apply(action) {
                Break(distance) => return Ok(distance.to_string()),
                Continue(state) => tracked_state = state,
            }
        }
        Err(AdventError::Other("No location visited twice".to_string()))
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
    #[case("R2, L3", "5")]
    #[case("R2, R2, R2", "2")]
    #[case("R5, L5, R5, R3", "12")]
    fn example1(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part1(&input)?, expected);
        Ok(())
    }

    #[rstest]
    #[case("R8, R4, R4, R8", "4")]
    fn example2(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part2(&input)?, expected);
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "301");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "130");
        Ok(())
    }
}

use std::collections::HashSet;
use nom::character::complete;
use nom::combinator::map;
use nom::multi::many1;
use aoc_common::prelude::*;
use aoc_data::prelude::*;

enum Move {
    Up,
    Down,
    Left,
    Right,
}

fn parse_move(i: &str) -> IResult<&str, Move> {
    map(
        complete::one_of("^v<>"),
        |c| match c {
            '^' => Move::Up,
            'v' => Move::Down,
            '<' => Move::Left,
            '>' => Move::Right,
            _ => unreachable!(),
        },
    ).parse(i)
}

fn parse_input(i: &str) -> Result<Vec<Move>> {
    many1(parse_move).parse(i).map_and_finish()
}

#[derive(Eq, PartialEq, Hash, Clone, Copy)]
struct Coordinate {
    x: i32,
    y: i32,
}

impl From<(i32, i32)> for Coordinate {
    fn from(value: (i32, i32)) -> Self {
        Coordinate { x: value.0, y: value.1 }
    }
}

impl Coordinate {
    fn new() -> Coordinate {
        Coordinate { x: 0, y: 0 }
    }
    fn new_at(x: i32, y: i32) -> Self {
        Coordinate { x, y }
    }
    fn moved(&self, mv: &Move) -> Coordinate {
        match mv {
            Move::Up => (self.x, self.y + 1),
            Move::Down => (self.x, self.y - 1),
            Move::Left => (self.x - 1, self.y),
            Move::Right => (self.x + 1, self.y),
        }.into()
    }
}

struct Santa {
    position: Coordinate,
    visited: HashSet<Coordinate>,
}

impl Santa {
    fn new() -> Santa {
        let position = Coordinate::new();
        let mut visited = HashSet::new();
        visited.insert(position);
        Santa { position, visited }
    }
    fn move_and_visit(&mut self, mv: &Move) {
        self.position = self.position.moved(mv);
        self.visited.insert(self.position);
    }
    fn visited_count(&self) -> usize {
        self.visited.len()
    }
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2015
    }

    fn day(&self) -> Day {
        Day::Day3
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let moves = parse_input(input)?;
        let mut santa = Santa::new();
        for mv in moves.iter() {
            santa.move_and_visit(mv);
        }
        Ok(santa.visited_count().to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let moves = parse_input(input)?;
        let mut santa = Santa::new();
        let mut robo = Santa::new();
        for (idx, mv) in moves.iter().enumerate() {
            match idx % 2 {
                0 => santa.move_and_visit(mv),
                1 => robo.move_and_visit(mv),
                _ => unreachable!(),
            }
        }
        let total_visited: HashSet<Coordinate> = santa
            .visited
            .union(&robo.visited)
            .cloned()
            .collect();
        Ok(total_visited.len().to_string())
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
    #[case(">", "2")]
    #[case("^>v<", "4")]
    #[case("^v^v^v^v^v", "2")]
    fn example1(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part1(&input)?, expected);
        Ok(())
    }

    #[rstest]
    #[case("^v", "3")]
    #[case("^>v<", "3")]
    #[case("^v^v^v^v^v", "11")]
    fn example2(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part2(&input)?, expected);
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "2565");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "2639");
        Ok(())
    }
}

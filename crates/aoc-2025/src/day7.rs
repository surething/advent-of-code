use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::branch::alt;
use nom::character::complete::{char, newline};
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::sequence::terminated;
use std::collections::HashSet;

enum Tile {
    Empty,
    Splitter,
    Entrance,
}

struct Manifold {
    grid: Vec<Vec<Tile>>,
}

impl Manifold {
    fn new(grid: Vec<Vec<Tile>>) -> Self {
        Self { grid }
    }
    fn start_column(&self) -> usize {
        self.grid[0]
            .iter()
            .position(|tile| matches!(tile, Tile::Entrance))
            .expect("no entrance found")
    }
    fn num_splits(&self) -> usize {
        let mut beams: HashSet<usize> = HashSet::new();
        let mut splits = 0;

        beams.insert(self.start_column());
        for row in &self.grid {
            let mut new_beams = HashSet::new();
            for &col in &beams {
                match &row[col] {
                    Tile::Empty | Tile::Entrance => {
                        new_beams.insert(col);
                    }
                    Tile::Splitter => {
                        splits += 1;
                        if col > 0 {
                            new_beams.insert(col - 1);
                        }
                        if col + 1 < row.len() {
                            new_beams.insert(col + 1);
                        }
                    }
                }
            }
            beams = new_beams;
        }

        splits
    }
    fn num_timelines(&self) -> usize {
        let mut splits = vec![0; self.grid[0].len()];

        for row in &self.grid {
            let mut new_splits = vec![0; self.grid[0].len()];
            for (col, &split) in splits.iter().enumerate() {
                match &row[col] {
                    Tile::Empty => {
                        new_splits[col] += split;
                    }
                    Tile::Entrance => {
                        new_splits[col] += 1;
                    }
                    Tile::Splitter => {
                        if col > 0 {
                            new_splits[col - 1] += split;
                        }
                        if col + 1 < row.len() {
                            new_splits[col + 1] += split;
                        }
                    }
                }
            }
            splits = new_splits;
        }

        splits.iter().sum()
    }
}

fn parse_manifold(i: &str) -> IResult<&str, Manifold> {
    map(
        many1(terminated(
            many1(alt((
                map(char('.'), |_| Tile::Empty),
                map(char('^'), |_| Tile::Splitter),
                map(char('S'), |_| Tile::Entrance),
            ))),
            opt(newline),
        )),
        Manifold::new,
    )
    .parse(i)
}

fn parse_input(input: &str) -> Result<Manifold> {
    parse_manifold(input).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2025
    }

    fn day(&self) -> Day {
        Day::Day7
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let manifold = parse_input(input)?;
        Ok(manifold.num_splits().to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let manifold = parse_input(input)?;
        Ok(manifold.num_timelines().to_string())
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
        assert_eq!(solver.solve_part1(&input)?, "21");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "40");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "1537");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "18818811755665");
        Ok(())
    }
}

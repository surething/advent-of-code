use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::branch::alt;
use nom::character::complete;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::{preceded, terminated};

#[derive(Debug)]
enum Rotation {
    Left(u32),
    Right(u32),
}

enum ClickStrategy {
    EndsOnZero,
    PassesZero,
}

struct Dial {
    notches: u32,
    position: u32,
    strategy: Option<ClickStrategy>,
    clicks: u32,
}

impl Dial {
    fn new(notches: u32, position: u32) -> Dial {
        let strategy = None;
        let clicks = 0;
        Dial {
            notches,
            position,
            strategy,
            clicks,
        }
    }
    fn with_strategy(mut self, strategy: ClickStrategy) -> Dial {
        self.strategy = Some(strategy);
        self
    }
    fn apply_rotations(mut self, rotations: &[Rotation]) -> Dial {
        for rotation in rotations {
            self.rotate(rotation);
        }
        self
    }
    fn apply_strategy(&mut self, rotation: &Rotation) {
        match self.strategy {
            Some(ClickStrategy::EndsOnZero) => {
                if self.position == 0 {
                    self.clicks += 1
                }
            }
            Some(ClickStrategy::PassesZero) => {
                let dist_to_0 = match rotation {
                    Rotation::Left(_) if self.position > 0 => self.position,
                    Rotation::Left(_) => self.notches - self.position,
                    Rotation::Right(_) => self.notches - self.position,
                };
                // Check if we pass zero
                self.clicks += match rotation {
                    Rotation::Left(value) if dist_to_0 <= (*value % self.notches) => 1,
                    Rotation::Right(value) if dist_to_0 <= (*value % self.notches) => 1,
                    _ => 0,
                };
                // Count full rotations
                self.clicks += match rotation {
                    Rotation::Left(value) => value / self.notches,
                    Rotation::Right(value) => value / self.notches,
                };
            }
            None => {}
        }
    }
    fn rotate(&mut self, rotation: &Rotation) {
        self.apply_strategy(rotation);
        self.position = match rotation {
            Rotation::Left(value) => {
                (self.position + self.notches - (value % self.notches)) % self.notches
            }
            Rotation::Right(value) => (self.position + value) % self.notches,
        };
    }
    fn clicks(&self) -> u32 {
        self.clicks
    }
}

fn parse_rotation(i: &str) -> IResult<&str, Rotation> {
    terminated(
        alt((
            preceded(complete::char('L'), complete::u32.map(Rotation::Left)),
            preceded(complete::char('R'), complete::u32.map(Rotation::Right)),
        )),
        opt(complete::newline),
    )
    .parse(i)
}

fn parse_input(i: &str) -> Result<Vec<Rotation>> {
    many1(parse_rotation).parse(i).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2025
    }

    fn day(&self) -> Day {
        Day::Day1
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let rotations = parse_input(input.trim())?;
        let dial = Dial::new(100, 50)
            .with_strategy(ClickStrategy::EndsOnZero)
            .apply_rotations(&rotations);
        Ok(dial.clicks().to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let rotations = parse_input(input.trim())?;
        let mut dial = Dial::new(100, 50)
            .with_strategy(ClickStrategy::PassesZero)
            .apply_rotations(&rotations);
        Ok(dial.clicks().to_string())
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
        assert_eq!(solver.solve_part1(&input)?, "3");
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
        assert_eq!(solver.solve_part1(&input)?, "1139");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "6684");
        Ok(())
    }
}

use std::collections::HashSet;
use nom::branch::alt;
use nom::character::complete;
use nom::character::complete::newline;
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::sequence::{preceded, terminated};
use aoc_common::prelude::*;
use aoc_data::prelude::*;

fn parse_positive(i: &str) -> IResult<&str, i32> {
    preceded(
        complete::char('+'),
        complete::i32,
    ).parse(i)
}

fn parse_negative(i: &str) -> IResult<&str, i32> {
    preceded(
        complete::char('-'),
        map(complete::i32, |i| -i),
    ).parse(i)
}

fn parse_separator(i: &str) -> IResult<&str, ()> {
    alt((
        newline.map(|_| ()),
        (complete::char(','), complete::space0).map(|_| ()),
    )).parse(i)
}

fn parse_delta(i: &str) -> IResult<&str, i32> {
    terminated(
        alt((
            parse_positive,
            parse_negative,
        )),
        opt(parse_separator),
    ).parse(i)
}

fn parse_input(i: &str) -> Result<Vec<i32>> {
    many1(parse_delta).parse(i).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2018
    }

    fn day(&self) -> Day {
        Day::Day1
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let deltas = parse_input(input.trim())?;
        let frequency: i32 = deltas.iter().sum();
        Ok(frequency.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let deltas = parse_input(input.trim())?;
        let mut seen = HashSet::new();
        let mut frequency = 0;
        let mut iteration = 0;
        seen.insert(frequency);
        for delta in deltas.iter().cycle() {
            iteration += 1;
            if iteration > 1_000_000 {
                return Err(AdventError::Other(format!("No frequency found within limit: {}", iteration)));
            }
            frequency += delta;
            if !seen.insert(frequency) {
                return Ok(frequency.to_string());
            }
        }
        unreachable!()
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
    #[case("+1, -2, +3, +1", "3")]
    #[case("+1, +1, +1", "3")]
    #[case("+1, +1, -2", "0")]
    #[case("-1, -2, -3", "-6")]
    fn example1(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part1(&input)?, expected);
        Ok(())
    }

    #[rstest]
    #[case("+1, -2, +3, +1", "2")]
    #[case("+1, -1", "0")]
    #[case("+3, +3, +4, -2, -4", "10")]
    #[case("-6, +3, +8, +5, -6", "5")]
    #[case("+7, +7, -2, -7, -4", "14")]
    fn example2(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part2(&input)?, expected);
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "472");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "66932");
        Ok(())
    }
}

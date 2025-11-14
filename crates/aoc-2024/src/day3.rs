use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete;
use nom::combinator::map;
use nom::multi::many1;
use nom::sequence::{delimited, preceded, separated_pair};

enum Op {
    Enable,
    Disable,
    Mul(i32, i32),
}

impl Op {
    fn eval(&self) -> i32 {
        match self {
            Op::Enable => 0,
            Op::Disable => 0,
            Op::Mul(a, b) => a * b,
        }
    }
}

fn parse_mul(i: &str) -> IResult<&str, Op> {
    delimited(
        tag("mul("),
        map(
            separated_pair(complete::i32, tag(","), complete::i32),
            |(a, b)| Op::Mul(a, b),
        ),
        tag(")"),
    )
    .parse(i)
}

fn parse_enable(i: &str) -> IResult<&str, Op> {
    map(tag("do()"), |_| Op::Enable).parse(i)
}

fn parse_disable(i: &str) -> IResult<&str, Op> {
    map(tag("don't()"), |_| Op::Disable).parse(i)
}

fn parse_ops(i: &str) -> IResult<&str, Op> {
    alt((
        parse_disable,
        parse_enable,
        parse_mul,
        preceded(take(1usize), parse_ops),
    ))
    .parse(i)
}

fn parse_input(i: &str) -> Result<Vec<Op>> {
    many1(parse_ops).parse(i).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2024
    }

    fn day(&self) -> Day {
        Day::Day3
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let ops = parse_input(input)?;
        let result: i32 = ops.iter().map(|op| op.eval()).sum();
        Ok(result.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let ops = parse_input(input)?;
        let mut enabled = true;
        let result: i32 = ops
            .into_iter()
            .filter(|op| match op {
                Op::Enable => {
                    enabled = true;
                    false
                }
                Op::Disable => {
                    enabled = false;
                    false
                }
                Op::Mul(_, _) => enabled,
            })
            .map(|op| op.eval())
            .sum();
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
        assert_eq!(solver.solve_part1(&input)?, "161");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "48");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "173529487");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "99532691");
        Ok(())
    }
}

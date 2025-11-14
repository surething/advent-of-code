use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::character::complete;
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::sequence::{separated_pair, terminated};
use std::collections::HashMap;

struct Pair {
    left: i32,
    right: i32,
}

impl Pair {
    fn new(left: i32, right: i32) -> Pair {
        Pair { left, right }
    }
}

fn parse_pair(i: &str) -> IResult<&str, Pair> {
    map(
        terminated(
            separated_pair(complete::i32, complete::space1, complete::i32),
            opt(complete::newline),
        ),
        |(left, right)| Pair::new(left, right),
    )(i)
}

fn parse_input(i: &str) -> Result<Vec<Pair>> {
    many1(parse_pair)(i).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {

    fn event(&self) -> Event {
        Event::Event2024
    }

    fn day(&self) -> Day {
        Day::Day1
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let pairs = parse_input(input)?;

        let list_1 = pairs.iter().map(|p| p.left).sorted();
        let list_2 = pairs.iter().map(|p| p.right).sorted();

        let distance: u32 = list_1.zip(list_2).map(|(l, r)| l.abs_diff(r)).sum();

        Ok(distance.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let pairs = parse_input(input)?;

        let occurrences = pairs
            .iter()
            .map(|p| p.right)
            .fold(HashMap::new(), |mut acc, x| {
                *acc.entry(x).or_insert(0) += 1;
                acc
            });

        let similarity: i32 = pairs
            .iter()
            .map(|p| p.left)
            .map(|x| x * occurrences.get(&x).unwrap_or(&0))
            .sum();

        Ok(similarity.to_string())
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
        assert_eq!(solver.solve_part1(&input)?, "11");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "31");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "2264607");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "19457120");
        Ok(())
    }
}

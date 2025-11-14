use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::character::complete;
use nom::combinator::{map, opt};
use nom::multi::{many1, separated_list1};
use nom::sequence::terminated;
use nom::{IResult, Parser};

struct Report {
    levels: Vec<i32>,
}

trait ReportChecks {
    fn all_diffs_nominal(&self, range: std::ops::RangeInclusive<i32>) -> bool;
}

impl ReportChecks for Vec<i32> {
    fn all_diffs_nominal(&self, range: std::ops::RangeInclusive<i32>) -> bool {
        self.iter().all(|diff| range.contains(diff))
    }
}

impl Report {
    fn new(levels: Vec<i32>) -> Report {
        Report { levels }
    }

    // TODO: refactor and avoid a collect_vec call for diffs

    /// A report is deemed safe if:
    /// 1. all levels are either increasing or decreasing
    /// 2. all inter-level differences differ by at least 1 and at most 3
    fn is_safe(&self) -> bool {
        let diffs = self
            .levels
            .iter()
            .tuple_windows()
            .map(|(a, b)| a - b)
            .collect_vec();
        diffs.all_diffs_nominal(1..=3) || diffs.all_diffs_nominal(-3..=-1)
    }

    /// A report is deemed safe if:
    /// 1. all levels are either increasing or decreasing
    /// 2. all inter-level differences differ by at least 1 and at most 3
    /// 3. if unsafe, removing a single level would make it safe
    fn is_problem_dampened_safe(&self) -> bool {
        self.levels.iter().enumerate().any(|(n, _)| {
            let diffs = self
                .levels
                .iter()
                .take(n)
                .chain(self.levels.iter().skip(n + 1))
                .tuple_windows()
                .map(|(a, b)| a - b)
                .collect_vec();
            diffs.all_diffs_nominal(1..=3) || diffs.all_diffs_nominal(-3..=-1)
        })
    }
}

fn parse_report(i: &str) -> IResult<&str, Report> {
    map(
        terminated(
            separated_list1(complete::space1, complete::i32),
            opt(complete::newline),
        ),
        Report::new,
    )(i)
}

fn parse_input(i: &str) -> Result<Vec<Report>> {
    many1(parse_report)(i).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2024
    }

    fn day(&self) -> Day {
        Day::Day2
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let reports = parse_input(input)?;
        let num_safe = reports.iter().filter(|r| r.is_safe()).count();
        Ok(num_safe.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let reports = parse_input(input)?;
        let num_safe = reports
            .iter()
            .filter(|r| r.is_safe() || r.is_problem_dampened_safe())
            .count();
        Ok(num_safe.to_string())
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
        assert_eq!(solver.solve_part1(&input)?, "2");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "4");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "660");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "689");
        Ok(())
    }
}

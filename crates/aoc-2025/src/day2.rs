use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::multi::{many1, separated_list1};
use nom::sequence::separated_pair;
use std::ops::Sub;

struct Range {
    start: u64,
    end: u64,
}

trait Subrepeating {
    fn subrepeating(&self, num: usize) -> bool;
    fn any_subrepeating(&self) -> bool;
}

impl Subrepeating for u64 {
    fn subrepeating(&self, num: usize) -> bool {
        let s = self.to_string();
        let len = s.len();
        let max_sublen = len / 2;

        if num == 0 || num > max_sublen || len % num != 0 {
            return false;
        }

        let repeats = len / num;
        let first_sub = &s[0..num];
        let mut all_match = true;
        for i in 1..repeats {
            let start = i * num;
            let end = start + num;
            let sub = &s[start..end];
            if sub != first_sub {
                all_match = false;
                break;
            }
        }

        all_match
    }
    fn any_subrepeating(&self) -> bool {
        let len = self.ilog10() as usize + 1;
        let max_sublen = len / 2;
        (1..=max_sublen).any(|n| self.subrepeating(n))
    }
}

impl Range {
    fn new(start: u64, end: u64) -> Range {
        Range { start, end }
    }
}

fn parse_range(i: &str) -> IResult<&str, Range> {
    separated_pair(complete::u64, tag("-"), complete::u64)
        .map(|(start, end)| Range::new(start, end))
        .parse(i)
}

fn parse_input(i: &str) -> Result<Vec<Range>> {
    separated_list1(tag(","), parse_range)
        .parse(i)
        .map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2025
    }

    fn day(&self) -> Day {
        Day::Day2
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let ranges = parse_input(input.trim())?;
        let sum: u64 = ranges
            .iter()
            .map(|range| {
                (range.start..=range.end)
                    .filter(|&i| i.ilog10() % 2 == 1)
                    .filter(|&i| i.subrepeating((i.ilog10() as usize + 1) / 2))
                    .sum::<u64>()
            })
            .sum();
        Ok(sum.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let ranges = parse_input(input.trim())?;
        let sum: u64 = ranges
            .iter()
            .map(|range| {
                (range.start..=range.end)
                    .filter(|&i| i.any_subrepeating())
                    .sum::<u64>()
            })
            .sum();
        Ok(sum.to_string())
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
        assert_eq!(solver.solve_part1(&input)?, "1227775554");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "4174379265");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "8576933996");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "25663320831");
        Ok(())
    }
}

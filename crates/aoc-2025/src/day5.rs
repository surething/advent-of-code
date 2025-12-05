use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;
use nom::combinator::{map, opt};
use nom::multi::many1;
use nom::sequence::{separated_pair, terminated};

type IngredientID = u64;
type Range = (IngredientID, IngredientID);

struct Inventory {
    fresh: Vec<Range>,
    available: Vec<IngredientID>,
}

fn merge(lhs: &Range, rhs: &Range) -> Option<Range> {
    if lhs.0 <= rhs.1 + 1 && rhs.0 <= lhs.1 + 1 {
        Some((std::cmp::min(lhs.0, rhs.0), std::cmp::max(lhs.1, rhs.1)))
    } else {
        None
    }
}

fn reduce(ranges: &[Range]) -> Vec<Range> {
    let mut sorted = ranges.to_vec();
    sorted.sort_by_key(|r| r.0);
    let mut reduced: Vec<Range> = Vec::new();

    for range in sorted {
        if let Some(last) = reduced.last_mut()
            && let Some(merged) = merge(last, &range)
        {
            *last = merged;
            continue;
        }
        reduced.push(range);
    }

    reduced
}

fn full_reduce(ranges: &[Range]) -> Vec<Range> {
    let mut current = ranges.to_vec();
    loop {
        let reduced = reduce(&current);
        if reduced.len() == current.len() {
            return reduced;
        }
        current = reduced;
    }
}

impl Inventory {
    fn new(fresh: Vec<Range>, available: Vec<IngredientID>) -> Inventory {
        Inventory { fresh, available }
    }
    fn num_fresh(&self) -> usize {
        self.available
            .iter()
            .filter(|id| {
                self.fresh
                    .iter()
                    .any(|(start, end)| *id >= start && *id <= end)
            })
            .count()
    }
    fn num_considered_fresh(&self) -> usize {
        let reduced_fresh = full_reduce(&self.fresh);
        reduced_fresh
            .iter()
            .fold(0, |acc, (start, end)| acc + (end - start + 1)) as usize
    }
}

fn parse_range(i: &str) -> IResult<&str, (IngredientID, IngredientID)> {
    terminated(
        separated_pair(complete::u64, tag("-"), complete::u64),
        opt(newline),
    )
    .parse(i)
}

fn parse_id(i: &str) -> IResult<&str, IngredientID> {
    terminated(complete::u64, opt(newline)).parse(i)
}

fn parse_inventory(i: &str) -> IResult<&str, Inventory> {
    map(
        separated_pair(many1(parse_range), newline, many1(parse_id)),
        |(fresh, available)| Inventory::new(fresh, available),
    )
    .parse(i)
}

fn parse_input(input: &str) -> Result<Inventory> {
    parse_inventory.parse(input).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2025
    }

    fn day(&self) -> Day {
        Day::Day5
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let inventory = parse_input(input.trim())?;
        Ok(inventory.num_fresh().to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let inventory = parse_input(input.trim())?;
        Ok(inventory.num_considered_fresh().to_string())
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
        assert_eq!(solver.solve_part2(&input)?, "14");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "635");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "369761800782619");
        Ok(())
    }
}

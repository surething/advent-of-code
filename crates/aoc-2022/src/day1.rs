use std::cmp::max;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::combinator::opt;
use nom::multi::{many1, separated_list1};
use nom::sequence::terminated;
use aoc_common::prelude::*;
use aoc_data::prelude::*;

struct Inventory {
    items: Vec<i32>,
}

impl Inventory {
    fn new(items: Vec<i32>) -> Self {
        Self { items }
    }
    fn total_calories(&self) -> i32 {
        self.items.iter().sum()
    }
}

fn parse_inventory(i: &str) -> IResult<&str, Inventory> {
    many1(
        terminated(
            complete::i32,
            opt(tag("\n")),
        )
    )
        .map(|items| Inventory::new(items))
        .parse(i)
}

fn parse_input(i: &str) -> Result<Vec<Inventory>> {
    separated_list1(
        tag("\n"),
        parse_inventory,
    ).parse(i).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2022
    }

    fn day(&self) -> Day {
        Day::Day1
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let inventories = parse_input(input.trim())?;
        let max_calories = inventories
            .iter()
            .map(|inv| inv.total_calories())
            .max()
            .unwrap_or(0);
        Ok(max_calories.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let inventories = parse_input(input.trim())?;
        let mut total_calories: Vec<i32> = inventories
            .iter()
            .map(|inv| inv.total_calories())
            .collect();
        total_calories.sort_unstable_by(|a, b| b.cmp(a));
        let top_three_sum: i32 = total_calories
            .iter()
            .take(3)
            .sum();
        Ok(top_three_sum.to_string())
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
        assert_eq!(solver.solve_part1(&input)?, "24000");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "45000");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "74198");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "209914");
        Ok(())
    }
}

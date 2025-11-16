use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::character::complete;
use nom::character::complete::newline;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::terminated;

fn parse_expense(i: &str) -> IResult<&str, i32> {
    terminated(complete::i32, opt(newline)).parse(i)
}

fn parse_input(i: &str) -> Result<Vec<i32>> {
    many1(parse_expense).parse(i).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2020
    }

    fn day(&self) -> Day {
        Day::Day1
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let expenses = parse_input(input.trim())?;
        expenses
            .into_iter()
            .combinations(2)
            .find(|c| c.iter().sum::<i32>() == 2020)
            .map(|c| c.iter().product::<i32>().to_string())
            .ok_or(AdventError::Other("No valid pair found".to_string()))
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let expenses = parse_input(input.trim())?;
        expenses
            .into_iter()
            .combinations(3)
            .find(|c| c.iter().sum::<i32>() == 2020)
            .map(|c| c.iter().product::<i32>().to_string())
            .ok_or(AdventError::Other("No valid triplet found".to_string()))
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
        assert_eq!(solver.solve_part1(&input)?, "514579");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "241861950");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "864864");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "281473080");
        Ok(())
    }
}

use nom::character::complete;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::terminated;
use aoc_common::prelude::*;
use aoc_data::prelude::*;

fn parse_depth(i: &str) -> IResult<&str, i32> {
    terminated(
        complete::i32,
        opt(complete::newline),
    )
    .parse(i)
}

fn parse_input(i: &str) -> Result<Vec<i32>> {
    many1(parse_depth).parse(i).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2021
    }

    fn day(&self) -> Day {
        Day::Day1
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let depths = parse_input(input.trim())?;
        let count = depths
            .iter()
            .tuple_windows()
            .filter(|(a, b)| b > a)
            .count();
        Ok(count.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let depths = parse_input(input.trim())?;
        let count = depths
            .iter()
            .tuple_windows()
            .map(|(a, b, c)| a + b + c)
            .tuple_windows()
            .filter(|(a, b)| b > a)
            .count();
        Ok(count.to_string())
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
        assert_eq!(solver.solve_part1(&input)?, "7");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "5");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "1374");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "1418");
        Ok(())
    }
}

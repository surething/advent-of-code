use aoc_common::prelude::*;
use aoc_data::prelude::*;

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2020
    }

    fn day(&self) -> Day {
        Day::Day5
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        Ok("0".to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        Ok("0".to_string())
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
        assert_eq!(solver.solve_part1(&input)?, "0");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "0");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "0");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "0");
        Ok(())
    }
}

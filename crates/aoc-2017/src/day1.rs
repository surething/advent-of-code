use aoc_common::prelude::*;
use aoc_data::prelude::*;

fn kernel(acc: u32, pair: (char, char)) -> Result<u32> {
    let (a, b) = pair;
    if a == b {
        match a.to_digit(10) {
            Some(v) => Ok(acc + v),
            None => Err(AdventError::Other(format!("Invalid digit: {}", a))),
        }
    } else {
        Ok(acc)
    }
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2017
    }

    fn day(&self) -> Day {
        Day::Day1
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let trimmed = input.trim();
        trimmed
            .chars()
            .chain(trimmed.chars().take(1))
            .tuple_windows()
            .try_fold(0, kernel)
            .map(|sum| sum.to_string())
      }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let trimmed = input.trim();
        let len = trimmed.len();
        trimmed
            .chars()
            .cycle()
            .skip(len / 2)
            .take(len)
            .zip(trimmed.chars())
            .try_fold(0, kernel)
            .map(|sum| sum.to_string())
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
    #[case("1122", "3")]
    #[case("1111", "4")]
    #[case("1234", "0")]
    #[case("91212129", "9")]
    fn example1(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part1(&input)?, expected);
        Ok(())
    }

    #[rstest]
    #[case("1212", "6")]
    #[case("1221", "0")]
    #[case("123425", "4")]
    #[case("123123", "12")]
    #[case("12131415", "4")]
    fn example2(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part2(&input)?, expected);
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "1228");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "1238");
        Ok(())
    }
}

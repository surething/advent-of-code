use std::ops::ControlFlow::{Break, Continue};
use aoc_common::prelude::*;
use aoc_data::prelude::*;

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2015
    }

    fn day(&self) -> Day {
        Day::Day1
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        input
            .trim()
            .chars()
            .try_fold(0, |acc, c| match c {
                '(' => Ok(acc + 1),
                ')' => Ok(acc - 1),
                c => Err(AdventError::InvalidInput(format!("Invalid character: {}", c))),
            })
            .map(|floor| floor.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let result = input
            .trim()
            .chars()
            .enumerate()
            .try_fold(0, |acc, (idx, c)| {
                let new_acc = match c {
                    '(' => acc + 1,
                    ')' => acc - 1,
                    c => return Break(Err(AdventError::InvalidInput(format!("Invalid character: {}", c)))),
                };
                match new_acc {
                    -1 => Break(Ok(idx as i32 + 1 - acc)),
                    _ => Continue(new_acc),
                }
            });
        match result {
            Break(res) => res.map(|i| i.to_string()),
            Continue(_) => Err(AdventError::Other("Never entered basement".to_string())),
        }
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
    #[case("(())", "0")]
    #[case("()()", "0")]
    #[case("(((", "3")]
    #[case("(()(()(", "3")]
    #[case("))(((((", "3")]
    #[case("())", "-1")]
    #[case("))(", "-1")]
    #[case(")))", "-3")]
    #[case(")())())", "-3")]
    fn example1(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part1(&input)?, expected);
        Ok(())
    }

    #[rstest]
    #[case(")", "1")]
    #[case("()())", "5")]
    #[case(")()()", "1")]
    #[case("()())()()", "5")]
    fn example2(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part2(&input)?, expected);
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "232");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "1783");
        Ok(())
    }
}

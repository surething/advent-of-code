use aoc_common::prelude::*;
use aoc_data::prelude::*;

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2015
    }

    fn day(&self) -> Day {
        Day::Day4
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let key = input.trim();
        for i in 0..1_000_000_000 {
            let test_string = format!("{}{}", key, i);
            let digest = md5::compute(test_string.as_bytes());
            if digest.0[0] == 0 && digest.0[1] == 0 && (digest.0[2] & 0xF0) == 0 {
                return Ok(i.to_string());
            }
        }
        Err(AdventError::Other(
            "No valid number found in range".to_string(),
        ))
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let key = input.trim();
        for i in 0..1_000_000_000 {
            let test_string = format!("{}{}", key, i);
            let digest = md5::compute(test_string.as_bytes());
            if digest.0[0] == 0 && digest.0[1] == 0 && digest.0[2] == 0 {
                return Ok(i.to_string());
            }
        }
        Err(AdventError::Other(
            "No valid number found in range".to_string(),
        ))
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
    #[case("abcdef", "609043")]
    #[case("pqrstuv", "1048970")]
    fn example1(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part1(&input)?, expected);
        Ok(())
    }

    #[rstest]
    #[case("ckczppom", "117946")]
    fn part1(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part1(&input)?, expected);
        Ok(())
    }

    #[rstest]
    #[case("ckczppom", "3938038")]
    fn part2(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part2(&input)?, expected);
        Ok(())
    }
}

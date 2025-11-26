use std::fmt::Display;
use aoc_common::prelude::*;
use aoc_data::prelude::*;

#[derive(Debug, PartialEq)]
enum Niceness {
    Nice,
    Naughty,
}

impl Display for Niceness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Niceness::Nice => write!(f, "nice"),
            Niceness::Naughty => write!(f, "naughty"),
        }
    }
}

trait Part1 {
    fn judge_part1(&self) -> Niceness;
    fn vowel_rule(&self) -> bool;
    fn double_letter_rule(&self) -> bool;
    fn forbidden_substrings_rule(&self) -> bool;
}

trait Part2 {
    fn judge_part2(&self) -> Niceness;
    fn pair_repeat_rule(&self) -> bool;
    fn repeat_with_one_between_rule(&self) -> bool;
}

impl Part1 for String {
    fn judge_part1(&self) -> Niceness {
        match (
            self.vowel_rule(),
            self.double_letter_rule(),
            self.forbidden_substrings_rule(),
        ) {
            (true, true, true) => Niceness::Nice,
            _ => Niceness::Naughty,
        }
    }
    fn vowel_rule(&self) -> bool {
        let vowels = ['a', 'e', 'i', 'o', 'u'];
        let count = self
            .chars()
            .filter(|c| vowels.contains(c))
            .count();
        count >= 3
    }
    fn double_letter_rule(&self) -> bool {
        self.chars()
            .tuple_windows()
            .any(|(a, b)| a == b)
    }
    fn forbidden_substrings_rule(&self) -> bool {
        let forbidden = ["ab", "cd", "pq", "xy"];
        !forbidden.iter().any(|&s| self.contains(s))
    }
}

impl Part2 for String {
    fn judge_part2(&self) -> Niceness {
        match (
            self.pair_repeat_rule(),
            self.repeat_with_one_between_rule(),
        ) {
            (true, true) => Niceness::Nice,
            _ => Niceness::Naughty,
        }
    }
    fn pair_repeat_rule(&self) -> bool {
        self.chars()
            .tuple_windows()
            .enumerate()
            .any(|(i, (a, b))| {
                self.chars()
                    .skip(i + 2)
                    .tuple_windows()
                    .any(|(c, d)| a == c && b == d)
            })
    }
    fn repeat_with_one_between_rule(&self) -> bool {
        self.chars()
            .tuple_windows()
            .any(|(a, _, c)| a == c)
    }
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2015
    }

    fn day(&self) -> Day {
        Day::Day5
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let count = input
            .lines()
            .map(String::from)
            .filter(|s| s.judge_part1() == Niceness::Nice)
            .count();
        Ok(count.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let count = input
            .lines()
            .map(String::from)
            .filter(|s| s.judge_part2() == Niceness::Nice)
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
    #[case("ugknbfddgicrmopn", "1")]
    #[case("aaa", "1")]
    #[case("jchzalrnumimnmhp", "0")]
    #[case("haegwjzuvuyypxyu", "0")]
    #[case("dvszwmarrgswjxmb", "0")]
    fn example1(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part1(&input)?, expected);
        Ok(())
    }

    #[rstest]
    #[case("qjhvhtzxzqqjkmpb", "1")]
    #[case("xxyxx", "1")]
    #[case("uurcxstgmygtbstg", "0")]
    #[case("ieodomkazucvgmuy", "0")]
    fn example2(solver: Solver, #[case] input: String, #[case] expected: String) -> Result<()> {
        assert_eq!(solver.solve_part2(&input)?, expected);
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "238");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "69");
        Ok(())
    }
}

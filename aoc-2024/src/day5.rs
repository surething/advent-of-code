use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::IResult;
use nom::bytes::complete::tag;
use nom::character::complete;
use nom::character::complete::newline;
use nom::combinator::{map, opt};
use nom::multi::{many0, many1, separated_list1};
use nom::sequence::{separated_pair, terminated};

type Page = u32;

#[derive(Debug)]
struct Rule {
    ante: Page,
    post: Page,
}

type Rules = Vec<Rule>;
type Update = Vec<Page>;
type Updates = Vec<Update>;

trait RuleChecking {
    fn check_rule(&self, rule: &Rule) -> bool;
    fn check_rules(&self, rules: &Rules) -> bool {
        rules.iter().all(|rule| self.check_rule(rule))
    }
}

trait RuleFixing {
    fn fix_with_rule(&mut self, rule: &Rule) -> bool;
    fn fix_with_rules(&mut self, rules: &Rules) {
        while rules.iter().any(|rule| self.fix_with_rule(rule)) {}
    }
}

trait MiddlePage {
    fn middle(&self) -> Page;
}

impl RuleChecking for Update {
    fn check_rule(&self, rule: &Rule) -> bool {
        let ante = self
            .iter()
            .find_position(|&&page| page == rule.ante)
            .map(|r| r.0);
        let post = self
            .iter()
            .find_position(|&&page| page == rule.post)
            .map(|r| r.0);
        if let (Some(ante_index), Some(post_index)) = (ante, post) {
            ante_index < post_index
        } else {
            true
        }
    }
}

impl RuleFixing for Update {
    fn fix_with_rule(&mut self, rule: &Rule) -> bool {
        let ante = self
            .iter()
            .find_position(|&&page| page == rule.ante)
            .map(|r| r.0);
        let post = self
            .iter()
            .find_position(|&&page| page == rule.post)
            .map(|r| r.0);
        if let (Some(ante_index), Some(post_index)) = (ante, post) {
            if ante_index >= post_index {
                self.remove(ante_index);
                self.insert(post_index, rule.ante);
                return true;
            }
        }
        false
    }
}

impl MiddlePage for Update {
    fn middle(&self) -> Page {
        self[self.len() / 2]
    }
}

fn parse_page(i: &str) -> IResult<&str, Page> {
    complete::u32(i)
}

fn parse_rule(i: &str) -> IResult<&str, Rule> {
    map(
        separated_pair(parse_page, tag("|"), parse_page),
        |(ante, post)| Rule { ante, post },
    )
    .parse(i)
}

fn parse_rules(i: &str) -> IResult<&str, Rules> {
    many1(terminated(parse_rule, newline)).parse(i)
}

fn parse_update(i: &str) -> IResult<&str, Update> {
    terminated(separated_list1(tag(","), parse_page), opt(newline)).parse(i)
}

fn parse_updates(i: &str) -> IResult<&str, Updates> {
    many1(parse_update).parse(i)
}

fn parse_raw(i: &str) -> IResult<&str, (Rules, Updates)> {
    let (i, rules) = parse_rules(i)?;
    let (i, _) = many0(newline).parse(i)?;
    let (_, updates) = parse_updates(i)?;
    Ok((i, (rules, updates)))
}

fn parse_input(i: &str) -> Result<(Rules, Updates)> {
    parse_raw(i).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2024
    }

    fn day(&self) -> Day {
        Day::Day5
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let (rules, updates) = parse_input(input)?;
        let result: Page = updates
            .iter()
            .filter(|update| update.check_rules(&rules))
            .map(|update| update.middle())
            .sum();
        Ok(result.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let (rules, updates) = parse_input(input)?;
        let result: Page = updates
            .into_iter()
            .filter(|update| !update.check_rules(&rules))
            .map(|mut update| {
                update.fix_with_rules(&rules);
                update.middle()
            })
            .sum();
        Ok(result.to_string())
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
        assert_eq!(solver.solve_part1(&input)?, "143");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "123");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "4185");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "4480");
        Ok(())
    }
}

use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::NomRange;
use nom::branch::alt;
use nom::character::complete;
use nom::character::complete::{anychar, newline, none_of, space0, space1};
use nom::combinator::{map, opt};
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, terminated};

enum Op {
    Add,
    Mul,
}

type Line = Vec<u64>;

struct Offsets {
    lines: Vec<String>,
    ops: String,
}

struct Homework {
    lines: Vec<Line>,
    ops: Vec<Op>,
}

fn kernel(data: &[u64], op: &Op) -> u64 {
    match op {
        Op::Add => data.iter().sum(),
        Op::Mul => data.iter().product(),
    }
}

impl Offsets {
    fn new(lines: Vec<String>, ops: String) -> Offsets {
        Offsets { lines, ops }
    }
    fn offset_for(&self, col: usize) -> Vec<usize> {
        // find col'th non-space character in the ops line
        let col_idx = self
            .ops
            .chars()
            .enumerate()
            .filter(|&(_, c)| c != ' ')
            .nth(col)
            .map(|(idx, _)| idx)
            .expect("Column index out of bounds in ops line");

        let mut offsets = Vec::new();
        for (row_idx, line) in self.lines.iter().enumerate() {
            let mut offset = 0;
            while let Some(' ') = line.chars().nth(col_idx + offset) {
                offset += 1;
            }
            offsets.push(offset);
        }
        offsets
    }
}

impl Homework {
    fn new(lines: Vec<Line>, ops: Vec<Op>) -> Homework {
        Homework { lines, ops }
    }
    fn column_total(&self, col: usize) -> u64 {
        let data = self.lines.iter().map(|line| line[col]).collect_vec();
        let op = &self.ops[col];
        kernel(&data, op)
    }
    fn correct_column_total(&self, col: usize, offsets: &Offsets) -> u64 {
        let data = self
            .lines
            .iter()
            .map(|line| line[col])
            .map(|v| v.to_string())
            .collect_vec();

        let max_len = data.iter().map(|s| s.len()).max().unwrap_or(0);

        let col_offset = offsets.offset_for(col);

        let corrected_data = max_len
            .bounded_iter()
            .map(|i| {
                data.iter()
                    .enumerate()
                    .map(|(r, s)| {
                        let offset = col_offset[r];
                        let mut corrected = String::new();
                        for _ in 0..offset {
                            corrected.push(' ');
                        }
                        corrected.push_str(s);
                        corrected
                    })
                    .filter_map(|val| val.chars().nth(i))
                    .collect::<String>()
                    .trim()
                    .parse::<u64>()
                    .expect("Failed to parse corrected value")
            })
            .collect_vec();

        let op = &self.ops[col];
        kernel(&corrected_data, op)
    }
    fn grand_total(&self) -> u64 {
        self.ops
            .len()
            .bounded_iter()
            .map(|col| self.column_total(col))
            .sum()
    }
    fn correct_grand_total(&self, offsets: &Offsets) -> u64 {
        self.ops
            .len()
            .bounded_iter()
            .map(|col| self.correct_column_total(col, offsets))
            .sum()
    }
}

fn parse_line(i: &str) -> IResult<&str, Line> {
    terminated(
        many1(delimited(space0, complete::u64, space0)),
        complete::newline,
    )
    .parse(i)
}

fn parse_ops(i: &str) -> IResult<&str, Vec<Op>> {
    many1(delimited(
        space0,
        alt((
            complete::char('+').map(|_| Op::Add),
            complete::char('*').map(|_| Op::Mul),
        )),
        space0,
    ))
    .parse(i)
}

fn parse_homework(i: &str) -> IResult<&str, Homework> {
    (many1(parse_line), parse_ops)
        .map(|(lines, ops)| Homework::new(lines, ops))
        .parse(i)
}

fn parse_offsets(i: &str) -> IResult<&str, Offsets> {
    let (i, all_lines) = terminated(
        separated_list1(
            newline,
            map(many1(none_of("\n")), |chars| chars.into_iter().collect()),
        ),
        opt(newline),
    )
    .parse(i)?;
    let lines = all_lines
        .iter()
        .take(all_lines.len().saturating_sub(1))
        .cloned()
        .collect();
    let ops = all_lines.last().cloned().unwrap_or_default();
    Ok((i, Offsets::new(lines, ops)))
}

fn parse_input(input: &str) -> Result<(Homework, Offsets)> {
    Ok((
        parse_homework.parse(input).map_and_finish()?,
        parse_offsets.parse(input).map_and_finish()?,
    ))
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2025
    }

    fn day(&self) -> Day {
        Day::Day6
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let (homework, _) = parse_input(input.trim())?;
        Ok(homework.grand_total().to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let (homework, offsets) = parse_input(input.trim())?;
        Ok(homework.correct_grand_total(&offsets).to_string())
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
        assert_eq!(solver.solve_part1(&input)?, "4277556");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "3263827");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "8108520669952");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "11708563470209");
        Ok(())
    }
}

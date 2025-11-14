use aoc_common::prelude::*;
use aoc_data::prelude::*;
use itertools::MinMaxResult;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{newline, satisfy};
use nom::character::is_newline;
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use std::collections::{HashMap, HashSet};
use std::ops::Index;

#[derive(Debug, EnumIter, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Coordinate {
    row: i32,
    col: i32,
}

type Coordinates = Vec<Coordinate>;

impl Coordinate {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
    fn neighbor(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self::new(self.row - 1, self.col),
            Direction::Down => Self::new(self.row + 1, self.col),
            Direction::Left => Self::new(self.row, self.col - 1),
            Direction::Right => Self::new(self.row, self.col + 1),
        }
    }
}

type Region = HashSet<Coordinate>;

struct Side {
    region: Region,
    direction: Direction,
}

trait RegionExt {
    fn area(&self) -> usize;
    fn perimeter(&self) -> usize;
}

impl RegionExt for Region {
    fn area(&self) -> usize {
        self.len()
    }
    fn perimeter(&self) -> usize {
        self.iter()
            .map(|c| {
                Direction::iter()
                    .filter(|d| {
                        let neighbor = c.neighbor(*d);
                        !self.contains(&neighbor)
                    })
                    .count()
            })
            .sum()
    }
}

type Crop = char;
type Plots = Vec<Vec<Crop>>;

struct Garden {
    plots: Plots,
    rows: usize,
    cols: usize,
}

fn decompose_into_regions(coords: Coordinates) -> Vec<Region> {
    let mut regions = vec![];
    let mut remaining = coords.into_iter().collect::<HashSet<_>>();

    while let Some(&c) = remaining.iter().next() {
        let mut region = HashSet::new();
        let mut stack = vec![c];
        while let Some(c) = stack.pop() {
            if region.insert(c) {
                remaining.remove(&c);
                for d in Direction::iter() {
                    let neighbor = c.neighbor(d);
                    if remaining.contains(&neighbor) {
                        stack.push(neighbor);
                    }
                }
            }
        }
        regions.push(region);
    }
    regions
}

enum Decompose {
    Row(i32),
    Col(i32),
}

fn decompose_into_sides(region: &Region, direction: Direction, decompose: Decompose) -> Vec<Side> {
    let filtered_region = region
        .iter()
        .filter(|c| match decompose {
            Decompose::Row(row) => c.row == row,
            Decompose::Col(col) => c.col == col,
        })
        .filter(|c| !region.contains(&c.neighbor(direction)))
        .cloned()
        .collect_vec();

    decompose_into_regions(filtered_region)
        .iter()
        .cloned()
        .map(|region| Side { region, direction })
        .collect_vec()
}

fn count_sides(region: &Region) -> usize {
    let minmax_row = region.iter().map(|c| c.row).minmax();
    let minmax_col = region.iter().map(|c| c.col).minmax();
    let rows = match minmax_row {
        MinMaxResult::NoElements => return 0,
        MinMaxResult::OneElement(row) => row..row + 1,
        MinMaxResult::MinMax(min, max) => min..max + 1,
    };
    let cols = match minmax_col {
        MinMaxResult::NoElements => return 0,
        MinMaxResult::OneElement(col) => col..col + 1,
        MinMaxResult::MinMax(min, max) => min..max + 1,
    };

    let up_sides = rows
        .clone()
        .flat_map(|row| decompose_into_sides(region, Direction::Up, Decompose::Row(row)))
        .count();

    let down_sides = rows
        .clone()
        .flat_map(|row| decompose_into_sides(region, Direction::Down, Decompose::Row(row)))
        .count();

    let left_sides = cols
        .clone()
        .flat_map(|col| decompose_into_sides(region, Direction::Left, Decompose::Col(col)))
        .count();

    let right_sides = cols
        .clone()
        .flat_map(|col| decompose_into_sides(region, Direction::Right, Decompose::Col(col)))
        .count();

    up_sides + down_sides + left_sides + right_sides
}

impl Garden {
    fn new(plots: Plots) -> Self {
        let rows = plots.len();
        let cols = plots.first().map_or(0, |row| row.len());
        assert!(plots.iter().all(|row| row.len() == cols));
        Garden { plots, rows, cols }
    }
    fn show(&self) {
        for row in &self.plots {
            for plot in row {
                print!("{} ", plot);
            }
            println!();
        }
    }
    fn show_regions(&self) {
        let regions = self.regions();
        for region in regions {
            println!(
                "Region: area {}, perimeter {}",
                region.area(),
                region.perimeter()
            );
            for c in region {
                println!("  {}: {:?}", self[&c], c);
            }
            println!("-----------");
        }
    }
    fn regions(&self) -> Vec<Region> {
        self.coordinates()
            .fold(
                HashMap::new(),
                |mut acc: HashMap<char, Vec<Coordinate>>, c| {
                    let crop = self[&c];
                    acc.entry(crop).or_default().push(c);
                    acc
                },
            )
            .into_values()
            .flat_map(decompose_into_regions)
            .collect()
    }
    fn price(&self) -> usize {
        self.regions()
            .iter()
            .map(|r| r.area() * r.perimeter())
            .sum()
    }
    fn bulk_price(&self) -> usize {
        self.regions()
            .iter()
            .map(|r| r.area() * count_sides(r))
            .sum()
    }
    fn coordinates(&self) -> CoordinateIterator {
        CoordinateIterator {
            current_index: 0,
            rows: self.rows,
            cols: self.cols,
        }
    }
    fn coordinate_is_valid(&self, c: &Coordinate) -> bool {
        (0..self.rows as i32).contains(&c.row) && (0..self.cols as i32).contains(&c.col)
    }
}

impl Index<&Coordinate> for Garden {
    type Output = Crop;

    fn index(&self, c: &Coordinate) -> &Self::Output {
        assert!(self.coordinate_is_valid(c));
        &self.plots[c.row as usize][c.col as usize]
    }
}

struct CoordinateIterator {
    current_index: usize,
    rows: usize,
    cols: usize,
}

impl Iterator for CoordinateIterator {
    type Item = Coordinate;
    fn next(&mut self) -> Option<Self::Item> {
        if self.current_index < self.rows * self.cols {
            let row = (self.current_index / self.cols) as i32;
            let col = (self.current_index % self.cols) as i32;
            self.current_index += 1;
            Some(Coordinate::new(row, col))
        } else {
            None
        }
    }
}

fn parse_garden(i: &str) -> IResult<&str, Garden> {
    let (input, plots) = separated_list1(newline, many1(satisfy(|c| !is_newline(c as u8))))(i)?;
    let garden = Garden::new(plots);
    Ok((i, garden))
}

fn parse_input(input: &str) -> Result<Garden> {
    parse_garden(input).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2024
    }

    fn day(&self) -> Day {
        Day::Day12
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let garden = parse_input(input)?;
        Ok(garden.price().to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let garden = parse_input(input)?;
        Ok(garden.bulk_price().to_string())
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
    fn example_tiny() -> Result<()> {
        let input = "AAAA\nBBCD\nBBCC\nEEEC";
        let garden = parse_input(input)?;
        assert_eq!(garden.price(), 140);
        assert_eq!(garden.bulk_price(), 80);
        Ok(())
    }

    #[rstest]
    fn example_topology() -> Result<()> {
        let input = "OOOOO\nOXOXO\nOOOOO\nOXOXO\nOOOOO";
        let garden = parse_input(input)?;
        assert_eq!(garden.price(), 772);
        assert_eq!(garden.bulk_price(), 436);
        Ok(())
    }

    #[rstest]
    fn example_e() -> Result<()> {
        let input = "EEEEE\nEXXXX\nEEEEE\nEXXXX\nEEEEE";
        let garden = parse_input(input)?;
        assert_eq!(garden.bulk_price(), 236);
        Ok(())
    }

    #[rstest]
    fn example_moebius() -> Result<()> {
        let input = "AAAAAA\nAAABBA\nAAABBA\nABBAAA\nABBAAA\nAAAAAA";
        let garden = parse_input(input)?;
        assert_eq!(garden.bulk_price(), 368);
        Ok(())
    }

    #[rstest]
    fn example1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example1)?;
        assert_eq!(solver.solve_part1(&input)?, "1930");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "1206");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "1371306");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "805880");
        Ok(())
    }
}

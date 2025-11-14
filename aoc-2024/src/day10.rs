use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::AsChar;
use nom::character::complete::{newline, satisfy};
use nom::combinator::map;
use nom::multi::{many1, separated_list1};
use std::collections::HashSet;
use std::ops::Index;

type Height = usize;
type Score = usize;
type Rating = usize;

#[derive(EnumIter)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Coordinate {
    row: i32,
    col: i32,
}

impl Coordinate {
    fn new(row: i32, col: i32) -> Self {
        Self { row, col }
    }
    fn moved(&self, direction: Direction) -> Self {
        match direction {
            Direction::Up => Self::new(self.row - 1, self.col),
            Direction::Down => Self::new(self.row + 1, self.col),
            Direction::Left => Self::new(self.row, self.col - 1),
            Direction::Right => Self::new(self.row, self.col + 1),
        }
    }
}

type Trail = Vec<Coordinate>;
type Trails = Vec<Trail>;

trait TrailExt {
    fn reaches(&self, map: &Map, height: Height) -> bool;
}
trait TrailsExt {
    fn score(&self, map: &Map, height: Height) -> usize;
}

impl TrailExt for Trail {
    fn reaches(&self, map: &Map, height: Height) -> bool {
        self.iter()
            .filter(|c| map.coordinate_is_valid(c))
            .any(|c| map[c] == height)
    }
}

impl TrailsExt for Trails {
    fn score(&self, map: &Map, height: Height) -> Score {
        self.iter()
            .filter(|trail| trail.reaches(map, height))
            .count()
    }
}

struct Map {
    heights: Vec<Vec<Height>>,
    rows: usize,
    cols: usize,
}

impl Map {
    fn new(heights: Vec<Vec<Height>>) -> Self {
        let rows = heights.len();
        let cols = heights.first().map_or(0, |row| row.len());
        assert!(heights.iter().all(|row| row.len() == cols));
        Self {
            heights,
            rows,
            cols,
        }
    }
    fn show(&self) {
        for row in &self.heights {
            for height in row {
                if height == &100 {
                    print!(". ");
                } else {
                    print!("{} ", height);
                }
            }
            println!();
        }
    }
    fn coordinates(&self) -> CoordinateIterator {
        CoordinateIterator {
            current_index: 0,
            rows: self.rows,
            cols: self.cols,
        }
    }
    fn coordinates_with_height(&self, height: Height) -> Vec<Coordinate> {
        self.coordinates().filter(|c| self[c] == height).collect()
    }
    fn coordinate_is_valid(&self, c: &Coordinate) -> bool {
        (0..self.rows as i32).contains(&c.row) && (0..self.cols as i32).contains(&c.col)
    }
    fn trails_from(&self, start: &Coordinate, allow_retrace: bool) -> Trails {
        let mut trails = vec![];
        let mut visited = match allow_retrace {
            false => Some(HashSet::new()),
            true => None,
        };

        fn visit(
            map: &Map,
            start: &Coordinate,
            visited: &mut Option<HashSet<Coordinate>>,
            trail: &mut Trail,
            trails: &mut Trails,
        ) {
            if let Some(visited) = visited {
                if visited.contains(start) {
                    return;
                }
                visited.insert(*start);
            }

            trail.push(*start);

            let current_height = map[start];

            let next_coordinates = Direction::iter()
                .map(|direction| start.moved(direction))
                .filter(|c| map.coordinate_is_valid(c))
                .filter(|c| current_height + 1 == map[c])
                .collect_vec();

            for next in &next_coordinates {
                visit(map, next, visited, trail, trails);
            }

            if next_coordinates.is_empty() {
                trails.push(trail.clone());
                trail.clear();
            }
        }

        visit(self, start, &mut visited, &mut vec![], &mut trails);

        trails
    }
}

impl Index<&Coordinate> for Map {
    type Output = Height;

    fn index(&self, c: &Coordinate) -> &Self::Output {
        assert!(self.coordinate_is_valid(c));
        &self.heights[c.row as usize][c.col as usize]
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

fn parse_map(i: &str) -> IResult<&str, Map> {
    let (i, heights) = separated_list1(
        newline,
        many1(map(satisfy(|c| AsChar::is_dec_digit(c as u8)), |c| {
            c.to_string().parse::<Height>().unwrap()
        })),
    )
    .parse(i)?;
    let map = Map::new(heights);
    Ok((i, map))
}

fn parse_input(input: &str) -> Result<Map> {
    parse_map(input).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2024
    }

    fn day(&self) -> Day {
        Day::Day10
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let map = parse_input(input)?;
        // map.show();
        let total_score: Score = map
            .coordinates_with_height(0)
            .into_iter()
            .map(|c| map.trails_from(&c, false))
            .map(|trails| trails.score(&map, 9))
            .sum();
        Ok(total_score.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let map = parse_input(input)?;
        // map.show();
        let total_rating: Score = map
            .coordinates_with_height(0)
            .into_iter()
            .map(|c| map.trails_from(&c, true))
            .map(|trails| trails.score(&map, 9))
            .sum();
        Ok(total_rating.to_string())
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
        assert_eq!(solver.solve_part1(&input)?, "36");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "81");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "629");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "1242");
        Ok(())
    }
}

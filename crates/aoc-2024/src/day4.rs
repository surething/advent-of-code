use aoc_common::prelude::*;
use aoc_data::prelude::*;
use std::collections::HashMap;
use std::ops::Index;

#[derive(Debug, EnumIter)]
enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {
    fn delta(&self) -> (i32, i32) {
        match self {
            Direction::North => (0, -1),
            Direction::NorthEast => (1, -1),
            Direction::East => (1, 0),
            Direction::SouthEast => (1, 1),
            Direction::South => (0, 1),
            Direction::SouthWest => (-1, 1),
            Direction::West => (-1, 0),
            Direction::NorthWest => (-1, -1),
        }
    }
}

struct Grid<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coordinate {
    row: usize,
    col: usize,
}

type Coordinates = Vec<Coordinate>;

impl Coordinate {
    fn new(row: usize, col: usize) -> Coordinate {
        Coordinate { row, col }
    }
    fn neighbor(&self, d: &Direction) -> Option<Coordinate> {
        let (dx, dy) = d.delta();
        let new_row = self.row as i32 + dy;
        let new_col = self.col as i32 + dx;
        if new_row >= 0 && new_col >= 0 {
            Some(Coordinate::new(new_row as usize, new_col as usize))
        } else {
            None
        }
    }

    fn mark_line(&self, d: &Direction, n: usize) -> Vec<Coordinate> {
        let mut current = *self;
        let mut result = vec![current];
        for _ in 1..n {
            current = match current.neighbor(d) {
                Some(c) => c,
                None => break,
            };
            result.push(current);
        }
        result
    }

    fn mark_lines(&self, n: usize) -> Vec<Coordinates> {
        Direction::iter()
            .map(|d| self.mark_line(&d, n))
            .collect_vec()
    }

    fn mark_x(&self, n: usize) -> Vec<Coordinates> {
        assert_eq!(n % 2, 1);
        let h = (n as i32) / 2;

        let mut line_1 = vec![];
        let mut line_2 = vec![];
        for i in -h..=h {
            let new_row = self.row as i32 + i;
            let new_col_1 = self.col as i32 + i;
            let new_col_2 = self.col as i32 - i;
            if new_row >= 0 && new_col_1 >= 0 {
                let c = Coordinate::new(new_row as usize, new_col_1 as usize);
                line_1.push(c);
            }
            if new_row >= 0 && new_col_2 >= 0 {
                let c = Coordinate::new(new_row as usize, new_col_2 as usize);
                line_2.push(c);
            }
        }

        let line_3 = line_1.iter().rev().copied().collect_vec();
        let line_4 = line_2.iter().rev().copied().collect_vec();
        vec![line_1, line_2, line_3, line_4]
    }
}

impl<T> Grid<T> {
    fn new(data: Vec<Vec<T>>) -> Grid<T> {
        let rows = data.len();
        let cols = data.first().map(|r| r.len()).unwrap_or(0);
        assert!(data.iter().all(|r| r.len() == cols));
        let data = data.into_iter().flatten().collect();
        Grid { rows, cols, data }
    }

    fn show(&self)
    where
        T: std::fmt::Display,
    {
        for row in 0..self.rows {
            for col in 0..self.cols {
                print!("{} ", self.data[row * self.cols + col]);
            }
            println!();
        }
    }

    fn search(&self, s: &[T]) -> usize
    where
        T: PartialEq + Copy,
    {
        // for each coordinate:
        //    for each direction:
        //       get all coordinates that match the string length radiating out
        //       filter for each radiant that matches the string
        //       collect the coordinates
        //    flatten the coordinates
        self.coordinates()
            .flat_map(|c| {
                c.mark_lines(s.len())
                    .into_iter()
                    .filter(|vc| {
                        let values = vc
                            .iter()
                            .filter(|c| self.coordinate_is_valid(c))
                            .map(|c| *self.coordinate_to_value(c))
                            .collect_vec();
                        values.as_slice() == s
                    })
                    .collect_vec()
            })
            .count()
    }

    fn search_x(&self, s: &[T]) -> usize
    where
        T: PartialEq + Copy,
    {
        assert_eq!(s.len() % 2, 1);
        assert!(!s.is_empty());
        self.coordinates()
            .flat_map(|c| {
                c.mark_x(s.len())
                    .into_iter()
                    .filter(|vc| {
                        let values = vc
                            .iter()
                            .filter(|c| self.coordinate_is_valid(c))
                            .map(|c| *self.coordinate_to_value(c))
                            .collect_vec();
                        values.as_slice() == s
                    })
                    .collect_vec()
            })
            .map(|v| v[s.len() / 2])
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0) += 1;
                acc
            })
            .iter()
            .filter(|&(_, v)| *v == 2)
            .count()
    }

    fn coordinates(&self) -> CoordinateIterator {
        CoordinateIterator {
            current_index: 0,
            rows: self.rows,
            cols: self.cols,
        }
    }

    fn coordinate_is_valid(&self, c: &Coordinate) -> bool {
        c.row < self.rows && c.col < self.cols
    }
    fn coordinate_to_index(&self, c: &Coordinate) -> usize {
        self.cols * c.row + c.col
    }
    fn coordinate_to_value(&self, c: &Coordinate) -> &T {
        let i = self.coordinate_to_index(c);
        &self.data[i]
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
            let row = self.current_index / self.cols;
            let col = self.current_index % self.cols;
            self.current_index += 1;
            Some(Coordinate { row, col })
        } else {
            None
        }
    }
}

impl<T> Index<Coordinate> for Grid<T> {
    type Output = T;

    fn index(&self, c: Coordinate) -> &Self::Output {
        let index = self.coordinate_to_index(&c);
        &self.data[index]
    }
}

type CharGrid = Grid<char>;

fn parse_input(i: &str) -> Result<CharGrid> {
    let data = i.lines().map(|l| l.trim().chars().collect()).collect_vec();
    Ok(CharGrid::new(data))
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2024
    }

    fn day(&self) -> Day {
        Day::Day4
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let grid = parse_input(input)?;
        // grid.show();
        let term = "XMAS".chars().collect_vec();
        let num = grid.search(&term);
        Ok(num.to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let grid = parse_input(input)?;
        let term = "MAS".chars().collect_vec();
        let num = grid.search_x(&term);
        Ok(num.to_string())
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
        assert_eq!(solver.solve_part1(&input)?, "18");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "9");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "2521");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "1912");
        Ok(())
    }
}

use crate::enums::{Day, Event};
use crate::error::Result;

pub trait Task {
    fn event(&self) -> Event;
    fn day(&self) -> Day;
    fn solve_part1(&self, input: &str) -> Result<String>;
    fn solve_part2(&self, input: &str) -> Result<String>;
}

#![allow(dead_code)]
use std::path::Path;

pub enum Event {
    Event2015,
    Event2016,
    Event2017,
    Event2018,
    Event2019,
    Event2020,
    Event2021,
    Event2022,
    Event2023,
    Event2024,
    Event2025,
}

pub enum Day {
    Day1,
    Day2,
    Day3,
    Day4,
    Day5,
    Day6,
    Day7,
    Day8,
    Day9,
    Day10,
    Day11,
    Day12,
    Day13,
    Day14,
    Day15,
    Day16,
    Day17,
    Day18,
    Day19,
    Day20,
    Day21,
    Day22,
    Day23,
    Day24,
    Day25,
}

pub enum Input {
    Example1,
    Example2,
    Part1,
    Part2,
}

impl Event {
    pub fn folder_name(&self) -> &Path {
        let name = match self {
            Event::Event2015 => "2015",
            Event::Event2016 => "2016",
            Event::Event2017 => "2017",
            Event::Event2018 => "2018",
            Event::Event2019 => "2019",
            Event::Event2020 => "2020",
            Event::Event2021 => "2021",
            Event::Event2022 => "2022",
            Event::Event2023 => "2023",
            Event::Event2024 => "2024",
            Event::Event2025 => "2025",
        };
        Path::new(name)
    }
}

impl Day {
    pub fn folder_name(&self) -> &Path {
        let name = match self {
            Day::Day1 => "day1",
            Day::Day2 => "day2",
            Day::Day3 => "day3",
            Day::Day4 => "day4",
            Day::Day5 => "day5",
            Day::Day6 => "day6",
            Day::Day7 => "day7",
            Day::Day8 => "day8",
            Day::Day9 => "day9",
            Day::Day10 => "day10",
            Day::Day11 => "day11",
            Day::Day12 => "day12",
            Day::Day13 => "day13",
            Day::Day14 => "day14",
            Day::Day15 => "day15",
            Day::Day16 => "day16",
            Day::Day17 => "day17",
            Day::Day18 => "day18",
            Day::Day19 => "day19",
            Day::Day20 => "day20",
            Day::Day21 => "day21",
            Day::Day22 => "day22",
            Day::Day23 => "day23",
            Day::Day24 => "day24",
            Day::Day25 => "day25",
        };
        Path::new(name)
    }
}

impl Input {
    pub fn file_name(&self) -> &Path {
        let name = match self {
            Input::Example1 => "example1.txt",
            Input::Example2 => "example2.txt",
            Input::Part1 => "part1.txt",
            Input::Part2 => "part2.txt",
        };
        Path::new(name)
    }
}

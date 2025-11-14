use aoc_common::prelude::*;
use aoc_data::prelude::*;
use nom::character::complete::one_of;
use nom::combinator::map_opt;
use nom::multi::many1;
use std::collections::HashSet;

type Size = usize;
type Id = usize;

#[derive(Copy, Clone)]
enum Desc {
    File(Size, Id),
    Space(Size),
}

#[derive(Debug, PartialEq, Eq)]
enum DefragAction {
    PartialMove,
    FullMove,
    PartialFill,
    Nothing,
}

struct DiskMap {
    disk: Vec<Desc>,
}

impl DiskMap {
    fn new(disk: Vec<Desc>) -> Self {
        Self { disk }
    }
    fn show_vertical(&self) {
        for desc in &self.disk {
            match desc {
                Desc::File(size, id) => println!("🟦 {}: {}", size, id),
                Desc::Space(size) => println!("⬜️ {} ", size),
            }
        }
    }
    fn show_horizontal(&self) {
        for desc in &self.disk {
            match desc {
                Desc::File(size, _) => {
                    for _ in 0..*size {
                        print!("🟦")
                    }
                }
                Desc::Space(size) => {
                    for _ in 0..*size {
                        print!("⬜️")
                    }
                }
            }
        }
        println!();
    }

    fn show_ids(&self) {
        for desc in &self.disk {
            match desc {
                Desc::File(size, id) => {
                    for _ in 0..*size {
                        print!("{} ", id)
                    }
                }
                Desc::Space(size) => {
                    for _ in 0..*size {
                        print!(". ")
                    }
                }
            }
        }
        println!();
    }

    fn checksum(&self) -> usize {
        let mut result = 0;
        let mut index = 0;
        for desc in &self.disk {
            match desc {
                Desc::File(size, id) => {
                    for i in 0..*size {
                        result += (index + i) * id;
                    }
                    index += size;
                }
                Desc::Space(size) => index += size,
            }
        }
        result
    }
    fn defrag_with_chunks(&mut self) {
        while self.defrag_chunk() != DefragAction::Nothing {}
    }
    fn defrag_chunk(&mut self) -> DefragAction {
        let index_space = self
            .disk
            .iter()
            .find_position(|desc| matches!(desc, Desc::Space(_)))
            .map(|(i, _)| i);
        let rev_index_file = self
            .disk
            .iter()
            .rev()
            .find_position(|desc| matches!(desc, Desc::File(_, _)))
            .map(|(i, _)| i);
        if let Some(index_space) = index_space {
            if let Some(rev_index_file) = rev_index_file {
                let index_file = self.disk.len() - rev_index_file - 1;
                return if index_space <= index_file {
                    self.move_file(index_file, index_space)
                } else {
                    DefragAction::Nothing
                };
            }
        }
        DefragAction::Nothing
    }
    fn defrag_with_files(&mut self) {
        let mut defragged_files = HashSet::new();
        loop {
            // Find the last file that isn't already marked as defragged
            let search_result = self.disk.iter().rev().find_position(|desc| match desc {
                Desc::File(_, id) => !defragged_files.contains(id),
                _ => false,
            });

            if let Some((rev_file_index, Desc::File(file_size, file_id))) = search_result {
                let file_index = self.disk.len() - rev_file_index - 1;
                defragged_files.insert(*file_id);
                let action = self.defrag_file(file_index, *file_size);
                // self.show_horizontal();
                // self.show_ids();
            } else {
                break;
            }
        }
    }
    fn defrag_file(&mut self, file_index: usize, file_size: usize) -> DefragAction {
        let search_result = self
            .disk
            .iter()
            .find_position(|desc| match desc {
                Desc::Space(space_size) => *space_size >= file_size,
                _ => false,
            })
            .map(|(i, _)| i);
        match search_result {
            Some(index_space) if file_index > index_space => {
                self.move_file(file_index, index_space)
            }
            Some(_) => DefragAction::Nothing,
            None => DefragAction::Nothing,
        }
    }
    fn move_file(&mut self, index_file: usize, index_space: usize) -> DefragAction {
        let disk = &mut self.disk;
        let desc_space = disk[index_space];
        let desc_file = disk[index_file];
        match (desc_space, desc_file) {
            (Desc::Space(size_space), Desc::File(size_file, id)) if size_space > size_file => {
                disk[index_space] = Desc::Space(size_space - size_file);
                disk.insert(index_space, Desc::File(size_file, id));
                disk[index_file + 1] = Desc::Space(size_file);
                DefragAction::PartialMove
            }
            (Desc::Space(size_space), Desc::File(size_file, id)) if size_space == size_file => {
                disk[index_space] = Desc::File(size_file, id);
                disk[index_file] = Desc::Space(size_file);
                DefragAction::FullMove
            }
            (Desc::Space(size_space), Desc::File(size_file, id)) => {
                disk[index_space] = Desc::File(size_space, id);
                disk[index_file] = Desc::File(size_file - size_space, id);
                DefragAction::PartialFill
            }
            _ => DefragAction::Nothing,
        }
    }
}

fn parse_disk_map(i: &str) -> IResult<&str, DiskMap> {
    let (i, sizes) = many1(map_opt(one_of("0123456789"), |c| c.to_digit(10))).parse(i)?;
    let disk = sizes
        .iter()
        .enumerate()
        .map(|(i, &size)| match i % 2 {
            0 => Desc::File(size as Size, i / 2),
            _ => Desc::Space(size as Size),
        })
        .filter(|desc| match desc {
            Desc::File(size, _) => *size > 0,
            Desc::Space(size) => *size > 0,
        })
        .collect_vec();
    Ok((i, DiskMap::new(disk)))
}

fn parse_input(input: &str) -> Result<DiskMap> {
    parse_disk_map(input).map_and_finish()
}

struct Solver {}

impl ResourceReader for Solver {}

impl Task for Solver {
    fn event(&self) -> Event {
        Event::Event2024
    }

    fn day(&self) -> Day {
        Day::Day9
    }

    fn solve_part1(&self, input: &str) -> Result<String> {
        let mut diskmap = parse_input(input)?;
        diskmap.defrag_with_chunks();
        Ok(diskmap.checksum().to_string())
    }

    fn solve_part2(&self, input: &str) -> Result<String> {
        let mut diskmap = parse_input(input)?;
        diskmap.defrag_with_files();
        Ok(diskmap.checksum().to_string())
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
        assert_eq!(solver.solve_part1(&input)?, "1928");
        Ok(())
    }

    #[rstest]
    fn example2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Example2)?;
        assert_eq!(solver.solve_part2(&input)?, "2858");
        Ok(())
    }

    #[rstest]
    fn part1(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part1)?;
        assert_eq!(solver.solve_part1(&input)?, "6386640365805");
        Ok(())
    }

    #[rstest]
    fn part2(solver: Solver) -> Result<()> {
        let input = solver.read_resource(Input::Part2)?;
        assert_eq!(solver.solve_part2(&input)?, "6423258376982");
        Ok(())
    }
}

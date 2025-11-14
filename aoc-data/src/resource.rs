use aoc_common::prelude::*;
use std::fs;
use std::path::{Path, PathBuf};

pub trait ResourceReader
where
    Self: Task,
{
    fn read_resource(&self, input: Input) -> Result<String> {
        let event = self.event();
        let day = self.day();
        read_resource(event, day, input)
    }
}

const MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");

fn read_resource(event: Event, day: Day, input: Input) -> Result<String> {
    let file_path: PathBuf = [
        Path::new(&MANIFEST_DIR),
        Path::new("resources"),
        event.folder_name(),
        day.folder_name(),
        input.file_name(),
    ]
    .iter()
    .collect();

    fs::read_to_string(file_path).map_err(AdventError::from)
}

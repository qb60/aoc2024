#![allow(dead_code)]

use std::fmt::{Debug, Display, Formatter};
use std::ops::{Deref, DerefMut};
use itertools::Itertools;
use BlockType::*;

pub fn day(input: &str) -> u64 {
    let mut blocks = parse_input(input).blocks;
    compress(&mut blocks);
    count_checksum(&blocks)
}

pub fn day_2(input: &str) -> u64 {
    let mut blocks = parse_input(input).blocks;
    defragmentate(&mut blocks);
    count_checksum(&blocks)
}

fn compress(blocks: &mut Blocks) {
    let mut current_block_index = 0;
    while current_block_index < blocks.len() {
        if blocks[current_block_index] == Empty {
            let last = loop {
                let last = blocks.pop().unwrap();
                if last != Empty {
                    break last;
                }
            };

            if current_block_index < blocks.len() {
                blocks[current_block_index] = last;
            } else {
                blocks.push(last);
            }
        }

        current_block_index += 1;
    }
}

#[derive(Debug, Default, PartialEq)]
struct File {
    id: u32,
    start: usize,
    size: usize,
}

fn defragmentate(blocks: &mut Blocks) {
    let files = get_file_list(blocks);

    for file in files.iter().rev() {
        if let Some(free_space_start) = find_free_space(blocks, file.size) {
            if free_space_start < file.start {
                move_blocks(blocks, file.start, free_space_start, file.size);
            }
        }
    }
}

fn find_free_space(blocks: &Blocks, needed_size: usize) -> Option<usize> {
    let mut start = None;
    let mut size = 0;

    for (i, block) in blocks.iter().enumerate() {
        match block {
            Empty if start.is_some() => {
                size += 1;
                if size == needed_size {
                    break;
                }
            }
            Empty => {
                start = Some(i);
                size = 1;
            }
            File(_) => {
                if size == needed_size {
                    break;
                }
                start = None;
            }
        }
    }

    if size != needed_size {
        return None;
    }

    start
}

fn move_blocks(blocks: &mut Blocks, src: usize, dest: usize, size: usize) {
    for i in 0..size {
        blocks[dest + i] = blocks[src + i];
        blocks[src + i] = Empty;
    }
}

fn get_file_list(blocks: &Blocks) -> Vec<File> {
    let mut files = Vec::new();

    let mut current_file: Option<File> = None;
    for (i, block) in blocks.iter().enumerate() {
        current_file = match (block, current_file) {
            (File(id), Some(file)) if file.id != *id => {
                files.push(file);
                Some(File {
                    id: *id,
                    start: i,
                    size: 1,
                })
            }
            (File(_), Some(mut file)) => {
                file.size += 1;
                Some(file)
            }
            (File(id), None) => {
                Some(File {
                    id: *id,
                    start: i,
                    size: 1,
                })
            }
            (Empty, Some(file)) => {
                files.push(file);
                None
            }
            _ => { None }
        }
    }

    if let Some(file) = current_file {
        files.push(file);
    }

    files
}

fn count_checksum(blocks: &Blocks) -> u64 {
    blocks.iter().enumerate().fold(0, |acc, (idx, block)| {
        if let File(id) = block {
            acc + idx as u64 * *id as u64
        } else {
            acc
        }
    })
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum BlockType {
    Empty,
    File(u32),
}

#[derive(PartialEq)]
struct Blocks(Vec<BlockType>);

impl Debug for Blocks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Blocks {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = self.iter().map(|block|
            match block {
                Empty => { ".".to_string() }
                File(id) if *id < 10 => { id.to_string() }
                File(_) => { "X".to_string() }
            }
        ).join("");
        write!(f, "{}", text)
    }
}

impl DerefMut for Blocks {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Deref for Blocks {
    type Target = Vec<BlockType>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq)]
struct Input {
    blocks: Blocks,
}

fn parse_input(input: &str) -> Input {
    let mut blocks = Vec::new();

    let mut last_file_id = 0;
    let mut current_block = File(last_file_id);

    // 2333133121414131402
    // 00...111...2...333.44.5555.6666.777.888899
    for char in input.trim().chars() {
        let size = char.to_digit(10).unwrap();
        blocks.append(&mut vec![current_block; size as usize]);

        (current_block, last_file_id) = match current_block {
            Empty => { (File(last_file_id), last_file_id) }
            File(_) => { (Empty, last_file_id + 1) }
        }
    }

    Input {
        blocks: Blocks(blocks),
    }
}

#[cfg(test)]
mod tests {
    use BlockType::{Empty, File};
    use super::*;

    impl From<&str> for Blocks {
        fn from(value: &str) -> Self {
            Blocks(value.trim().chars().map(|char| {
                match char {
                    '.' => Empty,
                    c => File(c.to_digit(10).unwrap())
                }
            }).collect())
        }
    }

    #[test]
    fn test_day() {
        let input = r#"2333133121414131402"#;

        assert_eq!(1928, day(input));
    }

    #[test]
    fn test_day_2() {
        let input = r#"2333133121414131402"#;

        assert_eq!(2858, day_2(input));
    }

    #[test]
    fn test_parse_input() {
        let input = r#"12345"#;

        let expected_parsed_input = Input {
            blocks: Blocks(vec![
                File(0),
                Empty, Empty,
                File(1), File(1), File(1),
                Empty, Empty, Empty, Empty,
                File(2), File(2), File(2), File(2), File(2)
            ]),
        };

        let actual_parsed_input = parse_input(input);

        assert_eq!(expected_parsed_input, actual_parsed_input);
    }

    #[test]
    fn test_text_to_blocks() {
        let text_blocks = "00...111...2...333.44.5555.6666.777.888899";

        let expected_blocks = Blocks(vec![
            File(0), File(0),
            Empty, Empty, Empty,
            File(1), File(1), File(1),
            Empty, Empty, Empty,
            File(2),
            Empty, Empty, Empty,
            File(3), File(3), File(3),
            Empty,
            File(4), File(4),
            Empty,
            File(5), File(5), File(5), File(5),
            Empty,
            File(6), File(6), File(6), File(6),
            Empty,
            File(7), File(7), File(7),
            Empty,
            File(8), File(8), File(8), File(8),
            File(9), File(9)
        ]);

        let actual_blocks = text_blocks.into();

        assert_eq!(expected_blocks, actual_blocks);
    }

    #[test]
    fn test_compress_blocks() {
        let mut blocks = Blocks::from("0..111....22222");
        let expected_blocks = Blocks::from("022111222");

        compress(&mut blocks);

        assert_eq!(expected_blocks, blocks);
    }

    #[test]
    fn test_compress_blocks_2() {
        let mut blocks = "00...111...2...333.44.5555.6666.777.888899".into();
        let expected_blocks: Blocks = "0099811188827773336446555566".into();

        println!("{}", &blocks);
        compress(&mut blocks);
        println!("{}", &expected_blocks);
        println!("{}", &blocks);

        assert_eq!(expected_blocks, blocks);
    }

    #[test]
    fn test_defragmentation() {
        let mut blocks = "00...111...2...333.44.5555.6666.777.888899".into();
        let expected_blocks: Blocks = "00992111777.44.333....5555.6666.....8888..".into();

        println!("{}", &blocks);
        defragmentate(&mut blocks);
        println!("{}", &expected_blocks);
        println!("{}", &blocks);

        assert_eq!(expected_blocks, blocks);
    }

    #[test]
    fn test_get_file_list() {
        use super::File;

        let blocks = "00...111...2...333.44.5555.6666.777.888899".into();

        let expected_files = vec![
            File { id: 0, start: 0, size: 2 },
            File { id: 1, start: 5, size: 3 },
            File { id: 2, start: 11, size: 1 },
            File { id: 3, start: 15, size: 3 },
            File { id: 4, start: 19, size: 2 },
            File { id: 5, start: 22, size: 4 },
            File { id: 6, start: 27, size: 4 },
            File { id: 7, start: 32, size: 3 },
            File { id: 8, start: 36, size: 4 },
            File { id: 9, start: 40, size: 2 },
        ];

        let actual_files = get_file_list(&blocks);

        assert_eq!(expected_files, actual_files);
    }

    #[test]
    fn test_find_free_space() {
        let blocks = "00...111..2....333.44".into();

        let space1 = find_free_space(&blocks, 3).unwrap();
        let space2 = find_free_space(&blocks, 2).unwrap();
        let space3 = find_free_space(&blocks, 4).unwrap();
        let space4 = find_free_space(&blocks, 5);

        assert_eq!(space1, 2);
        assert_eq!(space2, 2);
        assert_eq!(space3, 11);
        assert_eq!(space4, None);
    }

    #[test]
    fn test_move_file() {
        let mut blocks = "00...111..2....333.44".into();
        let expected_blocks: Blocks = "0044.111..2....333...".into();

        move_blocks(&mut blocks, 19, 2, 2);

        assert_eq!(expected_blocks, blocks);
    }
}

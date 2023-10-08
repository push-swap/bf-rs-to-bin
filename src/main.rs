extern crate regex;

use regex::Regex;
use std::iter::once;
use std::path::Path;
use std::{fs, io::Result, path::PathBuf, result, str::FromStr};

#[derive(Clone, Copy, Debug)]
pub enum Operation {
    PA = 0,
    PB = 1,
    SA = 2,
    SB = 3,
    SS = 4,
    RA = 5,
    RB = 6,
    RR = 7,
    RRA = 8,
    RRB = 9,
    RRR = 10,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> result::Result<Self, Self::Err> {
        match s.trim() {
            "pa" => Ok(Operation::PA),
            "pb" => Ok(Operation::PB),
            "sa" => Ok(Operation::SA),
            "sb" => Ok(Operation::SB),
            "ss" => Ok(Operation::SS),
            "ra" => Ok(Operation::RA),
            "rb" => Ok(Operation::RB),
            "rr" => Ok(Operation::RR),
            "rra" => Ok(Operation::RRA),
            "rrb" => Ok(Operation::RRB),
            "rrr" => Ok(Operation::RRR),
            _ => Err(()), // Invalid input
        }
    }
}

fn factorial(n: usize) -> usize {
    if n > 0 {
        n * factorial(n - 1)
    } else {
        1
    }
}

fn ps_hardcoded_find_index(arr: &[i32], len: usize) -> usize {
    if len == 0 {
        return 0;
    }

    let mut result = 0;
    for i in 0..len {
        if arr[0] > arr[i] {
            result += 1;
        }
    }

    (result * factorial(len - 1)) + ps_hardcoded_find_index(&arr[1..], len - 1)
}

fn process_file(path: &PathBuf) -> Result<()> {
    let contents = fs::read_to_string(path)?;
    let re = Regex::new(r#"Stack: "([^"]*)", Operations: "([^"]*)""#).unwrap();
    let mut index = 0;
    let mut data = Vec::new();
    let mut length = 0usize;
    for caps in re.captures_iter(&contents) {
        let input_part = &caps[1];
        let input = input_part
            .split_whitespace()
            .flat_map(|x| x.parse::<i32>())
            .collect::<Vec<_>>();
        let ops_part = &caps[2];
        assert_eq!(
            ps_hardcoded_find_index(input.as_slice(), input.len()),
            index
        );
        index += 1;
        let operations: Vec<Operation> = ops_part
            .split_whitespace()
            .filter_map(|s| s.parse::<Operation>().ok())
            .collect();
        length = input.len();
        data.push(operations);
    }

    let mut offset = 0u32;
    let mut ops = Vec::new();
    let mut offsets = Vec::new();
    for i in 0..factorial(length) {
        let operations = &data[i];
        offsets.push(offset);
        offset += (operations.len() + 1) as u32;
        ops.extend(
            operations
                .iter()
                .map(|op| *op as u8)
                .chain(once(11))
                .collect::<Vec<_>>(),
        );
    }
    let mut serialized_data = Vec::new();
    for offset in offsets {
        serialized_data.extend_from_slice(&offset.to_le_bytes());
    }
    serialized_data.extend(ops);

    let directory = "data";
    if !Path::new(directory).exists() {
        fs::create_dir(directory)?;
    }
    let file_path = format!(
        "{}/{}.bin",
        directory,
        path.file_name()
            .unwrap()
            .to_string_lossy()
            .strip_suffix(".txt")
            .unwrap()
    );
    fs::write(file_path, &serialized_data)?;

    Ok(())
}

fn main() {
    let directory_path = "generated";

    if let Ok(entries) = fs::read_dir(directory_path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if extension == "txt" {
                            process_file(&path)
                                .expect(format!("Failed to process {:?}", path).as_str());
                            println!(
                                "Successfully processed file {:?}",
                                path.file_name().unwrap()
                            );
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to read directory: {}", directory_path);
    }
}

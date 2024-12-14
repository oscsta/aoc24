use std::{fs, path::Path};

pub fn read(day: u8) -> String {
    let path_str = format!("./input/day{:0>2}_input.txt", day);
    let input_path = Path::new(&path_str);
    let input_str = fs::read_to_string(input_path).expect("Should find input files");
    input_str
}

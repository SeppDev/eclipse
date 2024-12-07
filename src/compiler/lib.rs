use super::path::Path;

const MODULE: &str = include_str!("lib/mod.ecl");

const IO: &str = include_str!("lib/io.ecl");

pub fn get_std_file(relative_file_path: &Path) -> String {
    let std = Path::from("std");

    if relative_file_path == &std.join("mod") {
        return MODULE.to_string();
    } else if relative_file_path == &std.join("io")  {
        return IO.to_string();
    }
    panic!("FAILED TO FIND {relative_file_path}")
}

use super::path::Path;

const MODULE: &str = include_str!("lib/mod.ecl");

const IO: &str = include_str!("lib/io.ecl");
const THREAD: &str = include_str!("lib/thread.ecl");
const MATH: &str = include_str!("lib/math.ecl");

pub fn get_std_file(relative_file_path: &Path) -> Option<String> {
    let std = Path::from("std");

    if relative_file_path == &std.join("mod") {
        return Some(MODULE.to_string());
    } else if relative_file_path == &std.join("io")  {
        return Some(IO.to_string());
    } else if relative_file_path == &std.join("thread")  {
        return Some(THREAD.to_string());
    } else if relative_file_path == &std.join("math")  {
        return Some(MATH.to_string());
    }
    return None;
}

use crate::common::path::Path;

use super::{
    errors::CompileResult,
    json::{self, JSON},
};

impl JSON {
    pub fn from_toml_source(mut source: String) -> CompileResult<Self> {
        Self::from_toml(JSON::new(), &mut source)
    }
    fn from_toml(table: JSON, source: &mut String) -> CompileResult<JSON> {
        let key = source;

        println!("{key:?}");

        todo!();
    }
}
// loop {
//     let line = match split.next() {
//         Some(l) => l,
//         None => break,
//     };
//     if line.len() == 0 {
//         continue;
//     }

//     let mut converted;
//     let is_array = if line.starts_with("[[") {
//         converted = line.replace("]]", "");
//         converted = converted.replace("[[", "");
//         true
//     } else if line.starts_with("[") {
//         converted = line.replace("[", "");
//         converted = converted.replace("]", "");
//         false
//     } else {
//         todo!()
//     };

//     let path = converted.split(".").collect::<Vec<&str>>();
//     if is_array {
//         todo!();
//         continue;
//     }
// }

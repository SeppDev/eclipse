// pub struct TOML {}

// impl JSON {
//     pub fn from_toml_source(source: String) -> CompileResult<Self> {
//         let mut chars = source.chars().peekable();
//         Self::from_toml(&mut chars)
//     }
//     fn from_toml(chars: &mut Peekable<Chars>) -> CompileResult<Self> {
//         let mut table = JSON::new();

//         // loop {
//         // if chars.peek() == &'[' {}

//         // let key = collect_until(chars, |c| c != &'=')
//         // let key = key.trim();
//         // if key.len() == 0 {
//         //     break;
//         // }

//         // chars.next();
//         // chars.skip_while(|c| c != &'\n').next();

//         // table.insert(key, JSON::Null);
//         // }

//         return Ok(table);
//     }
// }

// fn collect_until(chars: &mut Peekable<Chars>, predicate: impl FnOnce(&char) -> bool) -> String
// {
//     let mut string = String::new();
//     loop {
//         let char = match chars.next_if(predicate) {
//             Some(c) => c,
//             None => break,
//         };
//         string.push(char);
//     }
//     return string;
// }

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

use exit_code::ExitCode;

pub mod arguments;
pub mod counter;
pub mod errors;
pub mod exit_code;
pub mod files;
pub mod located;
pub mod path;
pub mod position;

pub mod json;
pub mod toml;

pub fn exit<T: ToString>(message: T, code: ExitCode) -> ! {
    println!("{}", message.to_string());
    std::process::exit(code as i32)
}

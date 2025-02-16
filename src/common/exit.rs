pub fn exit<T: ToString>(message: T) -> ! {
    println!("{}", message.to_string());
    std::process::exit(1)
}

pub fn exit<T: ToString>(message: T) -> ! {
    println!("error: {}", message.to_string());
    std::process::exit(1)
}

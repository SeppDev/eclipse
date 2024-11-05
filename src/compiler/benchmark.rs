pub fn benchmark<T, F>(function: F) -> (T, std::time::Duration)
where
    F: FnOnce() -> T,
{
    let start = std::time::Instant::now();
    let result = function();
    let duration = start.elapsed();

    (result, duration)
}

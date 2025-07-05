use std::env;

pub fn parse_args() -> (bool, bool) {
    let args: Vec<String> = env::args().collect();
    (
        contains(&args, "--round-fps"),
        contains(&args, "--fullscreen"),
    )
}

pub fn contains(vec: &[String], query: &str) -> bool {
    vec.contains(&query.to_string())
}

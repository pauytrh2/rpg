use std::env;

pub fn parse_args() -> bool {
    let args: Vec<String> = env::args().collect();
    args.contains(&"--round-fps".to_string())
}

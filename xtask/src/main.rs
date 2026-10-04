fn main() {
    let result = xtask::parse_args(std::env::args().skip(1))
        .and_then(|cli| xtask::run(cli, &xtask::workspace_root()));
    if let Err(error) = result {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

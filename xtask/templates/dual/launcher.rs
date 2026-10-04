use std::{path::Path, process::Command};

fn main() {
    let python = std::env::var_os("PYTHON")
        .unwrap_or_else(|| if cfg!(windows) { "python" } else { "python3" }.into());
    let script = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scripts/build_pair.py");
    let status = Command::new(python)
        .arg(script)
        .args(std::env::args_os().skip(1))
        .status()
        .unwrap_or_else(|error| {
            eprintln!("Could not start Python 3: {error}. Set PYTHON to its executable path.");
            std::process::exit(1);
        });
    std::process::exit(status.code().unwrap_or(1));
}

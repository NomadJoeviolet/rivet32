use std::{env, fs};

fn main() -> Result<(), String> {
    let args: Vec<_> = env::args().collect();
    if args.len() != 3 {
        return Err("usage: peripheral-elf-inspect ELF memory.x".into());
    }
    let bytes = fs::read(&args[1]).map_err(|e| e.to_string())?;
    let memory = fs::read_to_string(&args[2]).map_err(|e| e.to_string())?;
    let regions = xtask::elf::parse_memory_regions(&memory)?;
    let elf = xtask::elf::inspect_elf(&bytes, &regions)?;
    println!("{}", serde_json::json!({"elf": elf, "regions": regions}));
    Ok(())
}

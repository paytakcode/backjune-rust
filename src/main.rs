use std::collections::HashSet;
use std::fs;
use std::io::{self, BufRead, Write};

fn main() -> io::Result<()> {
    let base_dir = "bin";
    let toml_path = "Cargo.toml";

    let existing_bins = read_existing_bins(toml_path)?;

    let mut toml_file = fs::OpenOptions::new().append(true).open(toml_path)?;

    for entry in fs::read_dir(base_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension() == Some("rs".as_ref()) {
            if let Some(file_stem) = path.file_stem() {
                if let Some(name) = file_stem.to_str() {
                    if !existing_bins.contains(name) {
                        writeln!(
                            toml_file,
                            "[[bin]]\nname = \"{}\"\npath = \"{}/{}.rs\"\n",
                            name, base_dir, name
                        )?;
                    }
                }
            }
        }
    }

    Ok(())
}

fn read_existing_bins(toml_path: &str) -> io::Result<HashSet<String>> {
    let file = fs::File::open(toml_path)?;
    let reader = io::BufReader::new(file);

    let mut existing_bins = HashSet::new();
    let mut lines = reader.lines();
    while let Some(Ok(line)) = lines.next() {
        if line.starts_with("name =") {
            if let Some(name) = line.split('"').nth(1) {
                existing_bins.insert(name.to_string());
            }
        }
    }

    Ok(existing_bins)
}

use std::io::{self, Write};
use std::time::Instant;
use anyhow::Context;
use assert_cmd::Command;

pub fn get_bin_name(file_path: &str) -> String {
    let file_name = std::path::Path::new(file_path)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("unknown");
    if let Some(pos) = file_name.find('.') {
        file_name[..pos].to_string()
    } else {
        file_name.to_string()
    }
}

pub fn create_command(bin_name: &str) -> anyhow::Result<Command> {
    Command::cargo_bin(bin_name).context(format!("Failed to create command for {}", bin_name))
}

pub fn measure_execution_time<F>(func: F, test_number: Option<i32>) -> anyhow::Result<()>
where
    F: FnOnce() -> anyhow::Result<()>,
{
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    let start = Instant::now();
    let result = func();
    let duration = start.elapsed();

    match test_number {
        Some(c) => writeln!(handle, "Running time[{}]: {:?}", c, duration),
        None => writeln!(handle, "Running time: {:?}", duration),
    }
        .context("Failed to write running time to stdout")?;

    result
}
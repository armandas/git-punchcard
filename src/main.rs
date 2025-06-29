use anyhow::Result;
use chrono::DateTime;
use std::process::Command;

mod punchcard;
use crate::punchcard::Punchcard;

fn main() -> Result<()> {
    let output = Command::new("git")
        .args(["log", "--pretty=format:%cd"])
        .output()?;

    let timestamps = String::from_utf8(output.stdout)?
        .split_terminator('\n')
        .filter_map(|s| DateTime::parse_from_str(s, "%a %b %d %H:%M:%S%.3f %Y %z").ok())
        .map(|dt| dt.into())
        .collect();

    let punchcard = Punchcard::new(timestamps);
    println!("{punchcard}");

    Ok(())
}

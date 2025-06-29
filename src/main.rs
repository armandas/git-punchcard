use anyhow::Result;
use chrono::DateTime;
use clap::Parser;
use std::process::Command;

mod punchcard;
use crate::punchcard::Punchcard;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Filter by author name
    #[arg(long)]
    author: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let output = Command::new("git")
        .args(["log", "--pretty=format:%an,%cd"])
        .output()?;

    let output = String::from_utf8(output.stdout)?;
    let log = output
        .split_terminator('\n')
        .filter_map(|s| {
            if let Some((author, timestamp)) = s.split_once(',')
                && let Ok(dt) = DateTime::parse_from_str(timestamp, "%a %b %d %H:%M:%S%.3f %Y %z")
            {
                Some((author.to_string(), dt))
            } else {
                None
            }
        })
        .map(|(author, dt)| (author, dt.into()));

    let timestamps = if let Some(filter) = args.author {
        log.filter_map(|(author, dt)| {
            if author.to_lowercase().contains(&filter) {
                Some(dt)
            } else {
                None
            }
        })
        .collect()
    } else {
        log.map(|(_, dt)| dt).collect()
    };

    let punchcard = Punchcard::new(timestamps);
    println!("{punchcard}");

    Ok(())
}

#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]

/*
 * lookfor: find alternative
 * Copyright (C) 2024 DarkCeptor44
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use anyhow::{Result, anyhow};
use clap::{Parser, ValueEnum};
use colored::{Color, Colorize};
use rayon::prelude::*;
use std::{path::Path, process::exit};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author,version,about,long_about=None)]
struct App {
    #[arg(help = "Pattern to search for")]
    pattern: String,

    #[arg(long = "in", help = "Path to search in", default_value = ".")]
    path: String,

    #[arg(
        short,
        long,
        help = "Color of the highlighted text (off for no color)",
        default_value_t,
        value_enum
    )]
    color: Colors,

    #[arg(
        short = 'I',
        long = "case-sensitive",
        help = "Case sensitive search",
        default_value_t = false
    )]
    sensitive: bool,
}

#[derive(Clone, Default, ValueEnum)]
enum Colors {
    Red,
    Black,
    Green,
    Yellow,
    #[default]
    Blue,
    Magenta,
    Cyan,
    White,
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl From<Colors> for colored::Color {
    fn from(value: Colors) -> Self {
        match value {
            Colors::Red => Color::Red,
            Colors::Black => Color::Black,
            Colors::Green => Color::Green,
            Colors::Yellow => Color::Yellow,
            Colors::Blue => Color::Blue,
            Colors::Magenta => Color::Magenta,
            Colors::Cyan => Color::Cyan,
            Colors::White => Color::White,
            Colors::BrightBlack => Color::BrightBlack,
            Colors::BrightRed => Color::BrightRed,
            Colors::BrightGreen => Color::BrightGreen,
            Colors::BrightYellow => Color::BrightYellow,
            Colors::BrightBlue => Color::BrightBlue,
            Colors::BrightMagenta => Color::BrightMagenta,
            Colors::BrightCyan => Color::BrightCyan,
            Colors::BrightWhite => Color::BrightWhite,
        }
    }
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", format!("lookfor: {e:?}").red());
        exit(1);
    }
}

fn run() -> Result<()> {
    let args = App::parse();
    let path = args.path.as_path();
    let pattern = args.pattern.trim();
    let color = Color::from(args.color);

    if pattern.is_empty() {
        return Err(anyhow!("No pattern provided"));
    }

    if !path.is_dir() {
        return Err(anyhow!("Path is not a directory: {}", path.display()));
    }

    WalkDir::new(Path::new(path))
        .follow_links(true)
        .into_iter()
        .par_bridge()
        .filter_map(std::result::Result::ok)
        .for_each(|entry| {
            let path_str = entry.path().display().to_string();

            if path_str.is_empty() {
                return;
            }

            let path = if args.sensitive {
                path_str.clone()
            } else {
                path_str.to_lowercase()
            };

            if path.contains(pattern) {
                println!("{}", highlight_text(&path, pattern, color));
            }
        });

    Ok(())
}

fn highlight_text(text: &str, to_highlight: &str, color: Color) -> String {
    let index = text
        .to_lowercase()
        .find(&to_highlight.to_lowercase())
        .unwrap_or(0);
    format!(
        "{}{}{}",
        text[..index].normal(),
        text[index..index + to_highlight.len()].color(color).bold(),
        text[index + to_highlight.len()..].normal()
    )
}

#[cfg(test)]
mod tests {
    use std::{
        path::PathBuf,
        process::{Command, Stdio},
    };

    const BIN_PATH: &str = "target/debug/lookfor";

    #[test]
    fn test_pattern() {
        let mut cmd = Command::new(BIN_PATH);
        cmd.arg("clap");
        let output = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("Failed to run command");
        assert!(output.status.success());

        let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
        let expected = PathBuf::from("target")
            .join("debug")
            .join("deps")
            .join("clap_lex-");

        assert!(stdout.contains(&expected.display().to_string()));
    }

    #[test]
    fn test_pattern_not_found() {
        let mut cmd = Command::new(BIN_PATH);
        cmd.arg("Clap").arg("-I");
        let output = cmd
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .expect("Failed to run command");
        assert!(output.status.success());

        let stdout = String::from_utf8(output.stdout).expect("Invalid UTF-8");
        let expected = PathBuf::from("target")
            .join("debug")
            .join("deps")
            .join("clap_lex-");
        assert!(!stdout.contains(&expected.display().to_string()));
    }

    #[test]
    #[should_panic(expected = "lookfor: No pattern provided")]
    fn test_empty_pattern() {
        let mut cmd = Command::new(BIN_PATH);
        cmd.arg("");
        let output = cmd
            .stderr(Stdio::piped())
            .output()
            .expect("Failed to run command");

        assert!(!output.status.success());
        let stderr = String::from_utf8(output.stderr).expect("Invalid UTF-8");
        assert!(stderr.contains("lookfor: No pattern provided"));
    }
}

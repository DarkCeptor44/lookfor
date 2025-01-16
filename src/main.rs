#![forbid(unsafe_code)]

/**
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
use clap::{Parser, ValueEnum};
use colored::*;
use rayon::prelude::*;
use std::{collections::BTreeSet, path::Path, process::exit};
use walkdir::WalkDir;

#[derive(Parser)]
#[command(author,version,about,long_about=None)]
struct App {
    #[arg(help = "Pattern to search for")]
    pattern: String,

    #[arg(long = "in", help = "Path to search in", default_value = ".")]
    path: String,

    #[arg(
        long,
        help = "Color of the highlighted text",
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
    let args = App::parse();

    if args.pattern.is_empty() {
        println!("No pattern provided");
        exit(1);
    }

    if args.path.is_empty() {
        println!("No path provided");
        exit(1);
    }

    let color = Color::from(args.color);
    let pattern = match args.sensitive {
        true => args.pattern.to_owned(),
        false => args.pattern.to_lowercase(),
    };

    let found: BTreeSet<String> = WalkDir::new(Path::new(&args.path))
        .follow_links(true)
        .into_iter()
        .collect::<Vec<_>>()
        .par_iter()
        .fold(BTreeSet::new, |mut acc, e| {
            let e = e.as_ref().unwrap();
            let path_str = e.path().to_string_lossy().to_string();

            if path_str.is_empty() {
                return acc;
            }

            let path = match args.sensitive {
                true => path_str.to_owned(),
                false => path_str.to_lowercase(),
            };

            if path.contains(&pattern) {
                acc.insert(path_str);
            }

            acc
        })
        .reduce(BTreeSet::new, |mut a, b| {
            a.extend(b);
            a
        });

    for path in found {
        println!("{}", highlight_text(&path, &args.pattern, color));
    }
}

fn highlight_text(text: &str, to_highlight: &str, color: colored::Color) -> String {
    let index = text
        .to_lowercase()
        .find(&to_highlight.to_lowercase())
        .unwrap();
    format!(
        "{}{}{}",
        text[..index].normal(),
        text[index..index + to_highlight.len()].color(color).bold(),
        text[index + to_highlight.len()..].normal()
    )
}

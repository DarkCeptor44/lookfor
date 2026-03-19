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
use clap::Parser;
use colored::{Color, Colorize};
use crossbeam::channel::unbounded;
use lookfor::{SearchCtx, search_dir};
use std::{path::PathBuf, process::exit, sync::Arc};

#[derive(Parser)]
#[command(author,version,about,long_about=None)]
struct App {
    #[arg(help = "Pattern to search for")]
    pattern: String,

    #[arg(long = "in", help = "Path to search in", default_value = ".")]
    path: PathBuf,

    #[arg(
        short,
        long,
        help = "Color of the highlighted text (off or set NO_COLOR env var to disable)",
        default_value = "blue"
    )]
    color: Color,

    #[arg(
        short = 'I',
        long = "case-sensitive",
        help = "Case sensitive search",
        default_value_t
    )]
    sensitive: bool,
}

fn main() {
    if let Err(e) = main_impl() {
        eprintln!("{}", format!("lookfor: {e:?}").red());
        exit(1);
    }
}

fn main_impl() -> Result<()> {
    let args = App::parse();

    if args.pattern.is_empty() {
        return Err(anyhow!("pattern cannot be empty"));
    }

    let ctx = Arc::new(
        SearchCtx::new(args.pattern)
            .sensitive(args.sensitive)
            .color(args.color),
    );

    let (tx, rx) = unbounded();
    search_dir(&args.path, &ctx, &tx);

    while let Ok(path) = rx.try_recv() {
        println!("{path}");
    }

    Ok(())
}

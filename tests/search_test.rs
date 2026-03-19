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

use crate::common::setup_test_dirs;
use anyhow::Result;
use crossbeam::channel::unbounded;
use lookfor::{SearchCtx, search_dir};
use std::{fs::write, sync::Arc};

mod common;

#[test]
fn test_search_dir_sensitive() -> Result<()> {
    let temp_dir = setup_test_dirs(5, 2, 10);
    let temp_path = temp_dir.path().to_path_buf();

    let needle_path = temp_path.join("dir_0").join("target_file.match");
    write(&needle_path, b"data")?;

    let ctx = Arc::new(SearchCtx::new("target_file").sensitive(true));
    let (tx, rx) = unbounded();

    search_dir(&temp_path, &ctx, &tx);

    let mut found = false;
    while let Ok(res) = rx.try_recv() {
        if dbg!(res) == needle_path {
            found = true;
            break;
        }
    }

    assert!(
        found,
        "The search_dir function failed to find the target file"
    );
    Ok(())
}

#[test]
fn test_search_dir_insensitive() -> Result<()> {
    let temp_dir = setup_test_dirs(5, 2, 10);
    let temp_path = temp_dir.path().to_path_buf();

    let needle_path = temp_path.join("dir_0").join("target_file.match");
    write(&needle_path, b"data")?;

    let ctx = Arc::new(SearchCtx::new("target_file"));
    let (tx, rx) = unbounded();

    search_dir(&temp_path, &ctx, &tx);

    let mut found = false;
    while let Ok(res) = rx.try_recv() {
        if dbg!(res) == needle_path {
            found = true;
            break;
        }
    }

    assert!(
        found,
        "The search_dir function failed to find the target file"
    );
    Ok(())
}

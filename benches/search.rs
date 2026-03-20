/*
 * lookfor: find alternative
 * Copyright (C) 2026+ DarkCeptor44
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
use crossbeam::channel::unbounded;
use divan::{AllocProfiler, Bencher, black_box, counter::ItemsCount};
use lookfor::{SearchCtx, search_dir};
use std::{fs::write, sync::Arc};

#[path = "../tests/common/mod.rs"]
mod common;

#[global_allocator]
static ALLOC: AllocProfiler = AllocProfiler::system();

const PATTERN: &str = "target_file";

fn main() {
    divan::main();
}

#[divan::bench(name = "search_dir (1000 files, sensitive)")]
fn bench_search_1000_files(b: Bencher) {
    let (width, depth, files) = (10, 5, 20);
    let temp_dir = setup_test_dirs(width, depth, files);
    let total_items = width * depth * files;
    let temp_path = temp_dir.path().to_path_buf();

    let needle_path = temp_path.join("dir_0").join("target_file.match");
    write(&needle_path, b"data").unwrap();

    let (tx, rx) = unbounded();
    let ctx = Arc::new(SearchCtx::builder(PATTERN).sensitive(true).build().unwrap());

    b.counter(ItemsCount::new(total_items)).bench(|| {
        search_dir(&temp_path, &ctx, &tx);
    });

    while let Ok(res) = rx.try_recv() {
        black_box(res);
    }
}

#[divan::bench(name = "search_dir (1000 files, insensitive)")]
fn bench_search_1000_files_insensitive(b: Bencher) {
    let (width, depth, files) = (10, 5, 20);
    let temp_dir = setup_test_dirs(width, depth, files);
    let total_items = width * depth * files;
    let temp_path = temp_dir.path().to_path_buf();

    let needle_path = temp_path.join("dir_0").join("target_file.match");
    write(&needle_path, b"data").unwrap();

    let (tx, rx) = unbounded();
    let ctx = Arc::new(SearchCtx::new(PATTERN).unwrap());

    b.counter(ItemsCount::new(total_items)).bench(|| {
        search_dir(&temp_path, &ctx, &tx);
    });

    while let Ok(res) = rx.try_recv() {
        black_box(res);
    }
}

#[divan::bench(name = "search_dir (1000 files, glob)")]
fn bench_search_1000_files_glob(b: Bencher) {
    let (width, depth, files) = (10, 5, 20);
    let temp_dir = setup_test_dirs(width, depth, files);
    let total_items = width * depth * files;
    let temp_path = temp_dir.path().to_path_buf();

    let needle_path = temp_path.join("dir_0").join("target_file.match");
    write(&needle_path, b"data").unwrap();

    let (tx, rx) = unbounded();
    let ctx = Arc::new(SearchCtx::new("*.match").unwrap());

    b.counter(ItemsCount::new(total_items)).bench(|| {
        search_dir(&temp_path, &ctx, &tx);
    });

    while let Ok(res) = rx.try_recv() {
        black_box(res);
    }
}

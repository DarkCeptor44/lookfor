use crate::common::setup_test_dirs;
use crossbeam::channel::unbounded;
use divan::{AllocProfiler, Bencher, black_box, counter::ItemsCount};
use lookfor::{SearchCtx, search_dir};
use std::sync::Arc;

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
    let temp_dir = setup_test_dirs(width, depth, files).unwrap();
    let total_items = width * depth * files;
    let temp_path = temp_dir.path().to_path_buf();

    b.counter(ItemsCount::new(total_items))
        .with_inputs(|| temp_path.clone())
        .bench_values(|path| {
            let (tx, rx) = unbounded();
            let ctx = Arc::new(SearchCtx::new(PATTERN).sensitive(true));

            search_dir(&path, &ctx, &tx);

            while let Ok(res) = rx.try_recv() {
                black_box(res);
            }
        });
}

#[divan::bench(name = "search_dir (1000 files, insensitive)")]
fn bench_search_1000_files_insensitive(b: Bencher) {
    let (width, depth, files) = (10, 5, 20);
    let temp_dir = setup_test_dirs(width, depth, files).unwrap();
    let total_items = width * depth * files;
    let temp_path = temp_dir.path().to_path_buf();

    b.counter(ItemsCount::new(total_items))
        .with_inputs(|| temp_path.clone())
        .bench_values(|path| {
            let (tx, rx) = unbounded();
            let ctx = Arc::new(SearchCtx::new(PATTERN));

            search_dir(&path, &ctx, &tx);

            while let Ok(res) = rx.try_recv() {
                black_box(res);
            }
        });
}

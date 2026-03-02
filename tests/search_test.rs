use crate::common::setup_test_dirs;
use anyhow::Result;
use crossbeam::channel::unbounded;
use lookfor::{SearchCtx, search_dir};
use std::{fs::write, sync::Arc};

mod common;

#[test]
fn test_seach_dir_sensitive() -> Result<()> {
    let temp_dir = setup_test_dirs(5, 2, 10)?;
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
fn test_seach_dir_insensitive() -> Result<()> {
    let temp_dir = setup_test_dirs(5, 2, 10)?;
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

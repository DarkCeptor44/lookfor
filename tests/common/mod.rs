use anyhow::Result;
use std::fs::{create_dir_all, write};
use tempfile::{TempDir, tempdir};

pub fn setup_test_dirs(dirs: usize, depth: usize, files_per_dir: usize) -> Result<TempDir> {
    let temp_dir = tempdir()?;
    let root = temp_dir.path();

    for i in 0..dirs {
        let mut current_path = root.join(format!("dir_{i}"));

        for d in 0..depth {
            create_dir_all(&current_path)?;

            for f in 0..files_per_dir {
                let file_name = if i == dirs / 2 && d == depth / 2 && f == 0 {
                    "needle_match.txt".to_string()
                } else {
                    format!("file_{f}.log")
                };
                write(current_path.join(file_name), b"data")?;
            }

            current_path = current_path.join(format!("sub_{d}"));
        }
    }

    Ok(temp_dir)
}

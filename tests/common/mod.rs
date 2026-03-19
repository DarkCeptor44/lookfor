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

use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::fs::{create_dir_all, write};
use tempfile::{TempDir, tempdir};

pub fn setup_test_dirs(dirs: usize, depth: usize, files_per_dir: usize) -> TempDir {
    let temp_dir = tempdir().unwrap();
    let root = temp_dir.path();

    (0..dirs).into_par_iter().for_each(|i| {
        let mut current_path = root.join(format!("dir_{i}"));

        for d in 0..depth {
            create_dir_all(&current_path).unwrap();

            for f in 0..files_per_dir {
                let file_name = if i == dirs / 2 && d == depth / 2 && f == 0 {
                    "needle_match.txt".to_string()
                } else {
                    format!("file_{f}.log")
                };
                write(current_path.join(file_name), b"data").unwrap();
            }

            current_path = current_path.join(format!("sub_{d}"));
        }
    });

    temp_dir
}

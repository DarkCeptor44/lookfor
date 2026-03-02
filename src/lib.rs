#![forbid(unsafe_code)]
#![warn(clippy::pedantic, missing_debug_implementations)]

use colored::{Color, Colorize};
use crossbeam::channel::Sender;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::{path::Path, sync::Arc};

#[derive(Debug)]
pub struct SearchCtx {
    pattern: String,
    sensitive: bool,
    color: Option<Color>,
}

impl SearchCtx {
    #[must_use]
    pub fn new<S>(pattern: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            pattern: pattern.into(),
            sensitive: false,
            color: None,
        }
    }

    #[must_use]
    pub fn color<C>(mut self, color: C) -> Self
    where
        C: Into<Option<Color>>,
    {
        self.color = color.into();
        self
    }

    #[must_use]
    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = sensitive;
        self
    }
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

pub fn search_dir(path: &Path, ctx: &Arc<SearchCtx>, tx: &Sender<String>) {
    let Ok(read_dir) = std::fs::read_dir(path) else {
        return;
    };

    let entries: Vec<_> = read_dir.filter_map(Result::ok).collect();

    entries.into_par_iter().for_each(|entry| {
        let entry_path = entry.path();
        let path_str = entry_path.to_string_lossy();

        let is_match = if ctx.sensitive {
            path_str.contains(&ctx.pattern)
        } else {
            path_str
                .as_bytes()
                .windows(ctx.pattern.len())
                .any(|window| window.eq_ignore_ascii_case(ctx.pattern.as_bytes()))
        };

        if is_match {
            let result = if let Some(color) = ctx.color {
                highlight_text(&path_str, &ctx.pattern, color)
            } else {
                path_str.to_string()
            };
            let _ = tx.send(result);
        }

        if entry_path.is_dir() {
            search_dir(&entry_path, ctx, tx);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_highlight_text() {
        let text = "hello world";
        let to_highlight = "world";

        assert_eq!(
            highlight_text(text, to_highlight, Color::Blue),
            "hello \u{1b}[1;34mworld\u{1b}[0m"
        );
    }
}

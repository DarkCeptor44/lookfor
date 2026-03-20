#![forbid(unsafe_code)]
#![warn(clippy::pedantic, missing_debug_implementations)]

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

pub use colored;
pub use crossbeam;

use colored::{Color, Colorize};
use crossbeam::channel::Sender;
use globset::{GlobBuilder, GlobSet, GlobSetBuilder};
use rayon::iter::{ParallelBridge, ParallelIterator};
use std::{borrow::Cow, fmt::Debug, path::Path, sync::Arc};

/// Trait for fast lowercase conversion
pub trait FastLowercase {
    /// Converts a string to lowercase if at least one character is uppercase, otherwise just borrows it
    ///
    /// ## Returns
    ///
    /// A [Cow] (Copy-on-Write) string containing either the borrowed or the owned string
    ///
    /// ## Examples
    ///
    /// ```rust,no_run
    /// use lookfor::FastLowercase;
    ///
    /// let s1 = "Hello World";
    /// let s1_lower = s1.to_lowercase_fast(); // converts to lowercase which allocates a new String
    ///
    /// let s2 = "hello world";
    /// let s2_lower = s2.to_lowercase_fast(); // borrows s2 since its already lowercase
    /// ```
    fn to_lowercase_fast(&self) -> Cow<'_, str>;
}

impl<T> FastLowercase for T
where
    T: AsRef<str>,
{
    fn to_lowercase_fast(&self) -> Cow<'_, str> {
        let s = self.as_ref();
        if s.chars().all(|c| !c.is_uppercase()) {
            Cow::Borrowed(s)
        } else {
            Cow::Owned(s.to_lowercase())
        }
    }
}

/// Search context builder
#[derive(Debug, Clone)]
pub struct SearchCtxBuilder {
    color: Option<Color>,
    pattern: String,
    sensitive: bool,
}

impl SearchCtxBuilder {
    /// Creates a new [`SearchCtxBuilder`]
    ///
    /// ## Arguments
    ///
    /// * `pattern` - The pattern to search for
    ///
    /// ## Returns
    ///
    /// A new [`SearchCtxBuilder`]
    pub fn new<S>(pattern: S) -> Self
    where
        S: Into<String>,
    {
        Self {
            color: None,
            pattern: pattern.into(),
            sensitive: false,
        }
    }

    /// Sets the color to use for highlighting
    ///
    /// ## Arguments
    ///
    /// * `color` - The color to use (white is considered no color)
    ///
    /// ## Returns
    ///
    /// A new [`SearchCtxBuilder`]
    #[must_use]
    pub fn color<C>(mut self, color: C) -> Self
    where
        C: Into<Option<Color>>,
    {
        let color = color.into();
        self.color = if color.is_some_and(|c| c == Color::White) {
            None
        } else {
            color
        };
        self
    }

    /// Sets the pattern to search for
    ///
    /// ## Arguments
    ///
    /// * `pattern` - The pattern to search for
    ///
    /// ## Returns
    ///
    /// A new [`SearchCtxBuilder`]
    #[must_use]
    pub fn pattern<S>(mut self, pattern: S) -> Self
    where
        S: Into<String>,
    {
        self.pattern = pattern.into();
        self
    }

    /// Sets whether or not the pattern should be case-sensitive
    ///
    /// ## Arguments
    ///
    /// * `sensitive` - Whether or not the pattern should be case-sensitive
    ///
    /// ## Returns
    ///
    /// A new [`SearchCtxBuilder`]
    #[must_use]
    pub fn sensitive(mut self, sensitive: bool) -> Self {
        self.sensitive = sensitive;
        self
    }

    // END OF BUILDERS

    /// Builds the [`SearchCtx`]
    ///
    /// ## Returns
    ///
    /// A new [`SearchCtx`]
    ///
    /// ## Errors
    ///
    /// Returns an error if the glob pattern is invalid
    pub fn build(self) -> Result<SearchCtx, globset::Error> {
        let (pattern, sensitive) = (self.pattern, self.sensitive);

        let is_glob = pattern.chars().any(|c| c == '*' || c == '?' || c == '[');
        let glob_str = if is_glob {
            pattern.clone()
        } else {
            format!("*{pattern}*")
        };
        let glob = GlobBuilder::new(&glob_str)
            .case_insensitive(!sensitive)
            .build()?;

        let mut builder = GlobSetBuilder::new();
        builder.add(glob);

        Ok(SearchCtx {
            color: self.color,
            glob_pattern: builder.build()?,
            is_glob,
            pattern,
            sensitive,
        })
    }
}

/// Search context. Holds the pattern to search for, whether or not it should be case-sensitive, the color to use for highlighting, and the glob pattern.
///
/// Must be wrapped in an [Arc] (Atomic Reference Counted) to be shared between threads
#[derive(Clone)]
pub struct SearchCtx {
    /// The color to use for highlighting
    pub color: Option<Color>,

    /// The compiled glob pattern
    pub glob_pattern: GlobSet,

    /// The pattern to search for
    pub pattern: String,

    /// Whether or not the pattern is a glob
    pub is_glob: bool,

    /// Whether or not the pattern should be case-sensitive
    pub sensitive: bool,
}

impl Debug for SearchCtx {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SearchCtx")
            .field("color", &self.color)
            .field("pattern", &self.pattern)
            .field("is_glob", &self.is_glob)
            .field("sensitive", &self.sensitive)
            .finish_non_exhaustive()
    }
}

impl SearchCtx {
    /// Creates a new [`SearchCtxBuilder`]
    ///
    /// ## Arguments
    ///
    /// * `pattern` - The pattern to search for
    ///
    /// ## Returns
    ///
    /// A new [`SearchCtxBuilder`]
    pub fn builder<S>(pattern: S) -> SearchCtxBuilder
    where
        S: Into<String>,
    {
        SearchCtxBuilder::new(pattern)
    }

    /// Creates a new [`SearchCtx`] from a pattern
    ///
    /// If you need to change the default values, use [`SearchCtxBuilder`] or [`SearchCtx::builder`]
    ///
    /// ## Arguments
    ///
    /// * `pattern` - The pattern to search for
    ///
    /// ## Returns
    ///
    /// A new [`SearchCtx`]
    ///
    /// ## Errors
    ///
    /// Returns an error if the pattern is invalid
    pub fn new<S>(pattern: S) -> Result<SearchCtx, globset::Error>
    where
        S: Into<String>,
    {
        SearchCtxBuilder::new(pattern).build()
    }

    // END OF BUILDERS
}

/// Highlights a text with a given color
fn highlight_text(text: &str, to_highlight: &str, color: Color) -> String {
    let index = text
        .to_lowercase_fast()
        .find(&*to_highlight.to_lowercase_fast())
        .unwrap_or_default();
    format!(
        "{}{}{}",
        text[..index].normal(),
        text[index..index + to_highlight.len()].color(color),
        text[index + to_highlight.len()..].normal()
    )
}

/// Search a directory recursively for a pattern. Matches on both files and directories
///
/// ## Arguments
///
/// * `path` - The path to search
/// * `ctx` - The search context
/// * `tx` - The channel to send the results to
///
/// ## Examples
///
/// ```rust,no_run
/// use lookfor::{
///     crossbeam::channel::unbounded,
///     colored::Color,
///     SearchCtx,
///     search_dir,
/// };
/// use std::{path::Path, sync::Arc};
///
/// let ctx = Arc::new(SearchCtx::new("gurep").unwrap());
/// let (tx, rx) = unbounded();
///
/// let path = Path::new("path/to/search");
/// search_dir(&path, &ctx, &tx);
///
/// while let Ok(path) = rx.try_recv() {
///     println!("{}", path);
/// }
/// ```
pub fn search_dir(path: &Path, ctx: &Arc<SearchCtx>, tx: &Sender<String>) {
    let Ok(read_dir) = std::fs::read_dir(path) else {
        return;
    };

    read_dir.par_bridge().flatten().for_each(|entry| {
        let Ok(ft) = entry.file_type() else {
            return;
        };

        let entry_path = entry.path();
        if ft.is_dir() {
            search_dir(&entry_path, ctx, tx);
        }

        if let Some(fname) = entry.file_name().to_str() {
            if ctx.glob_pattern.is_match(fname) {
                let path_str = entry_path.to_string_lossy();
                let result = if let Some(color) = ctx.color {
                    if ctx.is_glob {
                        highlight_text(&path_str, fname, color)
                    } else {
                    highlight_text(&path_str, &ctx.pattern, color)
                    }
                } else {
                    path_str.into_owned()
                };
                let _ = tx.send(result);
            }
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
            "hello \u{1b}[34mworld\u{1b}[0m"
        );
    }
}

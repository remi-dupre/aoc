//! General purpose utilities.

use colored::*;

use std::cmp::min;
use std::fmt;
use std::iter;
use std::time::Duration;

// ---
// --- TryUnwrap: wrapper for Option and Result
// ---

pub trait TryUnwrap {
    type Val;
    fn try_unwrap(self) -> Result<Self::Val, String>;
}

impl<T> TryUnwrap for Option<T> {
    type Val = T;

    fn try_unwrap(self) -> Result<Self::Val, String> {
        if let Some(val) = self {
            Ok(val)
        } else {
            Err("empty output".to_string())
        }
    }
}

impl<T, E: fmt::Display> TryUnwrap for Result<T, E> {
    type Val = T;

    fn try_unwrap(self) -> Result<Self::Val, String> {
        self.map_err(|err| format!("{}", err))
    }
}

// ---
// --- Line: helper struct for printing
// ---

const DEFAULT_WIDTH: usize = 30;

/// Simple helper struct used to display lines with a dotted separator.
/// For example: "line text (1.2 ms) .............. status".
pub struct Line {
    text: String,
    duration: Option<Duration>,
    state: Option<ColoredString>,
}

impl Line {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            duration: None,
            state: None,
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn with_state(mut self, state: impl Into<ColoredString>) -> Self {
        self.state = Some(state.into());
        self
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_width = f.width().unwrap_or(DEFAULT_WIDTH);

        let duration = self
            .duration
            .map(|duration| format!(" ({:.2?})", duration))
            .unwrap_or_else(String::new);

        write!(f, "{}{}", self.text, duration.bright_black())?;

        if let Some(state) = &self.state {
            let width = self.text.chars().count() + 1 + duration.chars().count();
            let dots = display_width - min(display_width - 5, width) - 2;
            let dots: String = iter::repeat('.').take(dots).collect();
            write!(f, " {}", dots.bright_black())?;

            if state.contains('\n') {
                for line in state.trim_matches('\n').lines() {
                    write!(f, "\n    {}", line.bold())?;
                }
            } else {
                write!(f, " {}", state.clone().bold())?;
            }
        }

        Ok(())
    }
}

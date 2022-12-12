//! General purpose utilities.

use colored::*;

use std::cmp::min;
use std::fmt;
use std::time::Duration;

pub fn leak<T>(x: T) -> &'static mut T {
    Box::leak(Box::new(x))
}

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

const PREFIX: &str = "  ";
const DEFAULT_WIDTH: usize = 40;

#[derive(Clone, Copy)]
pub enum Status {
    Ok,
    Err,
    Warn,
}

/// Simple helper struct used to display lines with a dotted separator.
/// For example: "  - line text (1.2 ms) .............. status".
pub struct Line {
    text: String,
    duration: Option<Duration>,
    output: Option<ColoredString>,
    status: Option<Status>,
}

impl Line {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            duration: None,
            output: None,
            status: None,
        }
    }

    pub fn with_duration(mut self, duration: Duration) -> Self {
        self.duration = Some(duration);
        self
    }

    pub fn with_output(mut self, state: impl Into<ColoredString>) -> Self {
        self.output = Some(state.into());
        self
    }

    pub fn with_status(mut self, status: Status) -> Self {
        self.status = Some(status);
        self
    }

    pub fn println(&self) {
        println!("{self}");
    }
}

impl fmt::Display for Line {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_width = f.width().unwrap_or(DEFAULT_WIDTH);

        let duration = self
            .duration
            .map(|duration| format!(" ({:.2?})", duration))
            .unwrap_or_else(String::new);

        write!(f, "{PREFIX}{}{}", self.text, duration.bright_black())?;

        let show_status = |f: &mut fmt::Formatter| {
            if let Some(status) = self.status {
                let status_str = match status {
                    Status::Ok => "✓".green(),
                    Status::Err => "✗".red(),
                    Status::Warn => "⁉".yellow(),
                };

                write!(f, " {}", status_str)
            } else {
                Ok(())
            }
        };

        if let Some(state) = &self.output {
            let width = self.text.chars().count() + 1 + duration.chars().count();
            let dots = display_width - min(display_width - 5, width) - 2;
            let dots = ".".repeat(dots).bright_black();
            write!(f, " {dots}")?;
            show_status(f)?;

            if state.contains('\n') {
                for line in state.trim_matches('\n').lines() {
                    write!(f, "\n    {}", line.bold())?;
                }
            } else {
                write!(f, " {}", state.clone().bold())?;
            }
        } else {
            show_status(f)?;
        }

        Ok(())
    }
}

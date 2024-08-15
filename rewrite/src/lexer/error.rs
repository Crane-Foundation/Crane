//Make beautiful error messages with colour, context, and source snippets. with underlines

use std::fmt;
use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;

enum Type {
    Error,
    Warning,
}

pub struct ColorRed;
impl Color for ColorRed {
    fn wrap(s: String) -> String {
        format!("\x1b[31m{}\x1b[0m", s)
    }
}
pub struct ColorGreen;
impl Color for ColorGreen {
    fn wrap(s: String) -> String {
        format!("\x1b[32m{}\x1b[0m", s)
    }
}
pub struct ColorWhite;
impl Color for ColorWhite {
    fn wrap(s: String) -> String {
        format!("\x1b[37m{}\x1b[0m", s)
    }
}
pub struct ColorYellow;
impl Color for ColorYellow {
    fn wrap(s: String) -> String {
        format!("\x1b[33m{}\x1b[0m", s)
    }
}
trait Color {
    fn wrap(s: String) -> String;
}

#[derive(Debug, Clone)]
pub(super) enum ErrorType {
    UnclosedString,
    UnclosedComment,
    UnexpectedCharacter,
    UnexpectedToken,
    InvalidNumber,
    InvalidIdentifier,
    InvalidOperator,
    InvalidAssignment,
    ExtraToken,
}

pub fn underline(code: String, section: String) -> String {
    let mut underline = String::new();
    for c in code.chars() {
        if section.contains(c) {
            underline.push('^');
        } else {
            underline.push(' ');
        }
    }
    underline
}

pub fn colorize<T: Color>(s: String) -> String {
    T::wrap(s)
}

#[macro_export]
macro_rules! color {
    ($color:ident, $s:expr) => {
        $crate::error::colorize::<$color>($s.to_string())
    };
}

//error macro to take error, line number, error type, snippet and error precedence, and return a formatted error message
#[macro_export]
macro_rules! error {
    ($error:expr, $line:expr, $error_type:expr, $snippet:expr, $code:expr) => {{
        use $crate::error::*;
        //underline the error snippet
        let underline = $crate::color!(
            ColorGreen,
            $crate::error::underline($snippet.to_string(), $code.to_string())
        );
        //colorize the error snippet
        let snippet = $crate::color!(ColorWhite, $snippet);
        //colorize the error type
        let error_type = $crate::color!(ColorRed, format!("{:?}", $error_type));
        //colorize the line number
        let line = $crate::color!(ColorWhite, $line.to_string());
        //colorize the error message
        let error = $crate::color!(ColorRed, $error.to_string());
        //return the formatted error message
        let x = format!(
            "{}: {} at line {}\n{}\n{}",
            error_type, error, line, snippet, underline,
        );
        println!("{}", x);
        std::process::exit(0);
    }};
}

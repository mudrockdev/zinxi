#![allow(dead_code)]

#[derive(Clone, Copy)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
}

#[derive(Clone, Copy)]
pub enum Style {
    Normal,
    Bold,
    Dim,
    Italic,
    Underline,
}

pub fn fg(color: Color) -> &'static str {
    match color {
        Color::Black => "\x1b[30m",
        Color::Red => "\x1b[31m",
        Color::Green => "\x1b[32m",
        Color::Yellow => "\x1b[33m",
        Color::Blue => "\x1b[34m",
        Color::Magenta => "\x1b[35m",
        Color::Cyan => "\x1b[36m",
        Color::White => "\x1b[37m",
    }
}

pub fn bg(color: Color) -> &'static str {
    match color {
        Color::Black => "\x1b[40m",
        Color::Red => "\x1b[41m",
        Color::Green => "\x1b[42m",
        Color::Yellow => "\x1b[43m",
        Color::Blue => "\x1b[44m",
        Color::Magenta => "\x1b[45m",
        Color::Cyan => "\x1b[46m",
        Color::White => "\x1b[47m",
    }
}

pub fn style(s: Style) -> &'static str {
    match s {
        Style::Normal => "\x1b[0m",
        Style::Bold => "\x1b[1m",
        Style::Dim => "\x1b[2m",
        Style::Italic => "\x1b[3m",
        Style::Underline => "\x1b[4m",
    }
}

pub const RESET: &str = "\x1b[0m";

pub const BLUE_BOLD: &str = "\x1b[34m\x1b[1m";
pub const GREEN_BOLD: &str = "\x1b[32m\x1b[1m";
pub const YELLOW_BOLD: &str = "\x1b[33m\x1b[1m";
pub const RED_BOLD: &str = "\x1b[31m\x1b[1m";
pub const CYAN_BOLD: &str = "\x1b[36m\x1b[1m";
pub const MAGENTA_BOLD: &str = "\x1b[35m\x1b[1m";

// Semantic roles used across sections (inxi-like: bold section headers, colored keys).
pub const HEADER: &str = "\x1b[34m\x1b[1m"; // section headers, e.g. "System:"
pub const KEY: &str = "\x1b[36m\x1b[1m"; // field keys, e.g. "Kernel"

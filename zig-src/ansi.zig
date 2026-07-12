const std = @import("std");

pub const Ansi = struct {
    pub const Color = enum {
        black,
        red,
        green,
        yellow,
        blue,
        magenta,
        cyan,
        white,
    };

    pub const Style = enum {
        normal,
        bold,
        dim,
        italic,
        underline,
    };

    pub fn fg(color: Color) []const u8 {
        return switch (color) {
            .black => "\x1b[30m",
            .red => "\x1b[31m",
            .green => "\x1b[32m",
            .yellow => "\x1b[33m",
            .blue => "\x1b[34m",
            .magenta => "\x1b[35m",
            .cyan => "\x1b[36m",
            .white => "\x1b[37m",
        };
    }

    pub fn bg(color: Color) []const u8 {
        return switch (color) {
            .black => "\x1b[40m",
            .red => "\x1b[41m",
            .green => "\x1b[42m",
            .yellow => "\x1b[43m",
            .blue => "\x1b[44m",
            .magenta => "\x1b[45m",
            .cyan => "\x1b[46m",
            .white => "\x1b[47m",
        };
    }

    pub fn style(s: Style) []const u8 {
        return switch (s) {
            .normal => "\x1b[0m",
            .bold => "\x1b[1m",
            .dim => "\x1b[2m",
            .italic => "\x1b[3m",
            .underline => "\x1b[4m",
        };
    }

    pub const reset = "\x1b[0m";

    pub const blue_bold_str = "\x1b[34m\x1b[1m";
    pub const green_bold_str = "\x1b[32m\x1b[1m";
    pub const yellow_bold_str = "\x1b[33m\x1b[1m";
    pub const red_bold_str = "\x1b[31m\x1b[1m";
    pub const cyan_bold_str = "\x1b[36m\x1b[1m";
    pub const magenta_bold_str = "\x1b[35m\x1b[1m";
};

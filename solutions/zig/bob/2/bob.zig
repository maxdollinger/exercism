const std = @import("std");

const Response = struct {
    const CALM_DOWN = "Calm down, I know what I'm doing!";
    const SURE = "Sure.";
    const CHILL_OUT = "Whoa, chill out!";
    const FINE = "Fine. Be that way!";
    const WHATEVER = "Whatever.";
};

pub fn response(s: []const u8) []const u8 {
    const trimmed = std.mem.trim(u8, s, &std.ascii.whitespace);

    if (trimmed.len == 0) return Response.FINE;

    const question = trimmed[trimmed.len - 1] == '?';
    const yelling = isYelling: {
        var has_upper = false;
        for (trimmed) |c| {
            if (std.ascii.isLower(c)) break :isYelling false;
            if (std.ascii.isUpper(c)) has_upper = true;
        }

        break :isYelling has_upper;
    };

    if (yelling and question) return Response.CALM_DOWN;
    if (question) return Response.SURE;
    if (yelling) return Response.CHILL_OUT;

    return Response.WHATEVER;
}

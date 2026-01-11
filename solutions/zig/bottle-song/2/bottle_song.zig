const std = @import("std");

fn get_bottles(n: u4, cap: bool) []const u8 {
    const s = switch (n) {
        10 => "ten green bottles",
        9 => "nine green bottles",
        8 => "eight green bottles",
        7 => "seven green bottles",
        6 => "six green bottles",
        5 => "five green bottles",
        4 => "four green bottles",
        3 => "three green bottles",
        2 => "two green bottles",
        1 => "one green bottle",
        0 => "no green bottles",
        else => @panic("invalid verse number input"),
    };

    if (cap) {
        @constCast(s)[0] = std.ascii.toUpper(s[0]);
    } else {
        @constCast(s)[0] = std.ascii.toLower(s[0]);
    }

    return s;
}

fn write_verse(buffer: []u8, n: u4) !usize {
    const fst_part = get_bottles(n, true);
    const sec_part = get_bottles(n - 1, false);

    const fmt_str =
        \\{s} hanging on the wall,
        \\{s} hanging on the wall,
        \\And if one green bottle should accidentally fall,
        \\There'll be {s} hanging on the wall.
        \\
        \\
    ;

    const str = try std.fmt.bufPrint(buffer, fmt_str, .{ fst_part, fst_part, sec_part });

    return str.len;
}

pub fn recite(buffer: []u8, start_bottles: u4, take_down: u4) ![]const u8 {
    var written: usize = 0;
    for (0..take_down) |i| {
        const current_bottle = start_bottles - i;
        written += try write_verse(buffer[written..], current_bottle);
    }

    //Don't return trailing new lines
    written -= 2;
    return buffer[0..written];
}

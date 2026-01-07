const std = @import("std");

pub fn isPangram(str: []const u8) bool {
    if (str.len < 26) {
        return false;
    }

    var seen: u26 = 0;

    for (str) |c| {
        const offset = switch (c) {
            'a'...'z' => c - 'a',
            'A'...'Z' => c - 'A',
            else => continue,
        };

        seen |= @as(u26, 1) << @as(u5, @truncate(offset));
        if (seen == std.math.maxInt(u26)) {
            return true;
        }
    }

    return false;
}

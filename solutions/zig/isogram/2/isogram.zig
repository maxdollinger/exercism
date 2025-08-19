const std = @import("std");

pub fn isIsogram(str: []const u8) bool {
    var counts: [26]u8 = [_]u8{0} ** 26;

    for (str) |char| {
        const idx = switch (char) {
            'a'...'z' => char - 'a',
            'A'...'Z' => char - 'A',
            else => continue,
        };

        if (counts[idx] > 0) {
            return false;
        }

        counts[idx] += 1;
    }

    return true;
}

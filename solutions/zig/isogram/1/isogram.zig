const std = @import("std");

pub fn isIsogram(str: []const u8) bool {
    var counts: [26]u8 = [_]u8{0} ** 26;

    for (str) |char| {
        if (!std.ascii.isAlphabetic(char)) continue;

        const idx = if (char < 97) char - 65 else char - 97;

        if (counts[idx] > 0) {
            return false;
        }

        counts[idx] += 1;
    }

    return true;
}

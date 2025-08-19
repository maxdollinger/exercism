const std = @import("std");

pub fn isIsogram(str: []const u8) bool {
    var is_letter_present: [26]bool = undefined;

    for (str) |char| {
        const idx = switch (char) {
            'a'...'z' => char - 'a',
            'A'...'Z' => char - 'A',
            else => continue,
        };

        if (is_letter_present[idx]) {
            return false;
        }

        is_letter_present[idx] = true;
    }

    return true;
}

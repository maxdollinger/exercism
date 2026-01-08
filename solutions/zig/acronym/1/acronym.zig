const std = @import("std");
const mem = std.mem;

pub fn abbreviate(allocator: mem.Allocator, words: []const u8) mem.Allocator.Error![]u8 {
    var letters = try std.ArrayList(u8).initCapacity(allocator, 10);

    var last: u8 = '_';
    for (words) |c| {
        const isWordBound = !std.ascii.isAlphabetic(last) and last != '\'';
        if (isWordBound and std.ascii.isAlphabetic(c)) {
            try letters.append(allocator, std.ascii.toUpper(c));
        }
        last = c;
    }

    return letters.toOwnedSlice(allocator);
}

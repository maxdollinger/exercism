const std = @import("std");
const mem = std.mem;

pub fn abbreviate(allocator: mem.Allocator, words: []const u8) mem.Allocator.Error![]u8 {
    var letters = try std.ArrayList(u8).initCapacity(allocator, 10);

    var tokens = std.mem.tokenizeAny(u8, words, " _-");
    while (tokens.next()) |word| {
        try letters.append(allocator, std.ascii.toUpper(word[0]));
    }

    return letters.toOwnedSlice(allocator);
}

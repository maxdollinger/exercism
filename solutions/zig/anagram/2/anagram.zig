const std = @import("std");
const mem = std.mem;

fn get_char_count(word: []const u8) [26]u8 {
    var char_count = [_]u8{0} ** 26;
    for (word) |c| {
        const value = std.ascii.toLower(c) - 'a';
        char_count[value] += 1;
    }

    return char_count;
}

/// Returns the set of strings in `candidates` that are anagrams of `word`.
/// Caller owns the returned memory.
pub fn detectAnagrams(
    allocator: mem.Allocator,
    word: []const u8,
    candidates: []const []const u8,
) !std.BufSet {
    var set = std.BufSet.init(allocator);
    errdefer set.deinit();
    const word_cnt = get_char_count(word);

    for (candidates) |candidate| {
        if (std.ascii.eqlIgnoreCase(word, candidate)) continue;

        const cand_cnt = get_char_count(candidate);
        if (mem.eql(u8, &word_cnt, &cand_cnt)) try set.insert(candidate);
    }

    return set;
}

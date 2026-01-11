const std = @import("std");
const mem = std.mem;

fn char_count(word: []const u8) [26]u8 {
    var counts = [_]u8{0} ** 26;
    for (word) |c| {
        const value = std.ascii.toLower(c) - 'a';
        counts[value] += 1;
    }

    return counts;
}

pub fn detectAnagrams(
    allocator: mem.Allocator,
    word: []const u8,
    candidates: []const []const u8,
) !std.BufSet {
    var anagrams = std.BufSet.init(allocator);
    errdefer anagrams.deinit();

    const word_cnt = char_count(word);

    for (candidates) |candidate| {
        if (std.ascii.eqlIgnoreCase(word, candidate)) continue;

        const cand_cnt = char_count(candidate);
        if (mem.eql(u8, &word_cnt, &cand_cnt)) try anagrams.insert(candidate);
    }

    return anagrams;
}

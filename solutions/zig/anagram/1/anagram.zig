const std = @import("std");
const mem = std.mem;

fn word_to_vec(word: []const u8) [26]u8 {
    var char_count = [_]u8{0} ** 26;
    for (word) |c| {
        const value = std.ascii.toLower(c) - 'a';
        char_count[value] += 1;
    }

    return char_count;
}

fn equal_vecs(v1: [26]u8, v2: [26]u8) bool {
    inline for (0..26) |i| {
        if (v1[i] != v2[i]) return false;
    }

    return true;
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
    const word_vec = word_to_vec(word);

    for (candidates) |cand| {
        if (std.ascii.eqlIgnoreCase(word, cand)) continue;

        const cand_vec = word_to_vec(cand);
        if (equal_vecs(word_vec, cand_vec)) try set.insert(cand);
    }

    return set;
}

const std = @import("std");
/// Writes a reversed copy of `s` to `buffer`.
///
pub fn reverse(buffer: []u8, s: []const u8) []u8 {
    for (0..s.len) |i| {
        const idx = s.len - 1 - i;
        buffer[i] = s[idx];
    }

    return buffer[0..s.len];
}

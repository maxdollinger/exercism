const std = @import("std");
const mem = std.mem;

pub fn transform(allocator: mem.Allocator, legacy: std.AutoHashMap(i5, []const u8)) mem.Allocator.Error!std.AutoHashMap(u8, i5) {
    var new_map = std.AutoHashMap(u8, i5).init(allocator);
    errdefer new_map.deinit();

    var legacy_iter = legacy.iterator();
    while (legacy_iter.next()) |e| {
        for (e.value_ptr.*) |c| {
            try new_map.put(std.ascii.toLower(c), e.key_ptr.*);
        }
    }

    return new_map;
}

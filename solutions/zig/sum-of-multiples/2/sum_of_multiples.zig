const std = @import("std");
const mem = std.mem;

pub fn sum(allocator: mem.Allocator, factors: []const u32, limit: u32) !u64 {
    var set = std.AutoHashMap(u64, bool).init(allocator);
    defer set.deinit();

    var total: u64 = 0;

    for (factors) |f| {
        if (f <= 0) continue;
        var mult = f;
        while (mult < limit) : (mult += f) {
            if (!set.contains(mult)) {
                try set.put(mult, true);
                total += mult;
            }
        }
    }

    return total;
}

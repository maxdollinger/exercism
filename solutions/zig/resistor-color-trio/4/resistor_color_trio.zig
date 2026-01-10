const std = @import("std");
const mem = std.mem;

pub const ColorBand = enum(usize) {
    black = 0,
    brown = 1,
    red = 2,
    orange = 3,
    yellow = 4,
    green = 5,
    blue = 6,
    violet = 7,
    grey = 8,
    white = 9,
};

const ResistanceUnit = enum {
    ohms,
    kiloohms,
    megaohms,
    gigaohms,

    fn divisor(self: ResistanceUnit) f64 {
        return switch (self) {
            .gigaohms => 1_000_000_000.0,
            .megaohms => 1_000_000.0,
            .kiloohms => 1_000.0,
            .ohms => 1.0,
        };
    }

    fn suffix(self: ResistanceUnit) []const u8 {
        return @tagName(self);
    }
};

pub fn colorCode(colors: []const ColorBand) usize {
    const first = if (colors.len > 0) @intFromEnum(colors[0]) else 0;
    const second = if (colors.len > 1) @intFromEnum(colors[1]) else 0;
    const multiplier = if (colors.len > 2) @intFromEnum(colors[2]) else 0;

    return (first * 10 + second) * std.math.pow(usize, 10, multiplier);
}

pub fn label(allocator: mem.Allocator, colors: []const ColorBand) mem.Allocator.Error![]u8 {
    const value = @as(f64, @floatFromInt(colorCode(colors)));

    var buffer = try std.ArrayList(u8).initCapacity(allocator, 12);
    defer buffer.deinit(allocator);

    const units = std.enums.values(ResistanceUnit);
    for (units) |unit| {
        const scaled = value / unit.divisor();
        if (scaled < 1e3) {
            try buffer.print(allocator, "{d} {s}", .{ scaled, unit.suffix() });
            break;
        }
    }

    return buffer.toOwnedSlice(allocator);
}

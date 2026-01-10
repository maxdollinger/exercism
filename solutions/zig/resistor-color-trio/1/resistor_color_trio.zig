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

pub fn colorCode(colors: [3]ColorBand) usize {
    return (@intFromEnum(colors[0]) * 10 + @intFromEnum(colors[1])) * std.math.pow(usize, 10, @intFromEnum(colors[2]));
}

pub fn label(allocator: mem.Allocator, colors: []const ColorBand) mem.Allocator.Error![]u8 {
    var buffer = try std.ArrayList(u8).initCapacity(allocator, 30);
    defer buffer.deinit(allocator);

    var codes = [_]ColorBand{ ColorBand.black, ColorBand.black, ColorBand.brown };
    for (0..3) |i| {
        if (i > 2) break;
        codes[i] = colors[i];
    }

    const value = colorCode(codes);
    if (value == 0) {
        try buffer.print(allocator, "0 ohms", .{});
    } else {
        const digits = std.math.log10(value);
        switch (digits) {
            0...2 => try buffer.print(allocator, "{d} ohms", .{value}),
            3...5 => try buffer.print(allocator, "{d} kiloohms", .{@as(f64, @floatFromInt(value)) / 1_000.0}),
            6...8 => try buffer.print(allocator, "{d} megaohms", .{@as(f64, @floatFromInt(value)) / 1_000_000.0}),
            else => try buffer.print(allocator, "{d} gigaohms", .{@as(f64, @floatFromInt(value)) / 1_000_000_000.0}),
        }
    }

    return buffer.toOwnedSlice(allocator);
}

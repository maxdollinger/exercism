const std = @import("std");

pub fn convert(buffer: []u8, n: u32) []const u8 {
    var writer = std.Io.Writer.fixed(buffer);

    if (n % 3 == 0) {
        writer.writeAll("Pling") catch {};
    }
    if (n % 5 == 0) {
        writer.writeAll("Plang") catch {};
    }
    if (n % 7 == 0) {
        writer.writeAll("Plong") catch {};
    }

    writer.flush() catch unreachable;
    // If nothing was written, write the number
    if (buffer.len == 0) {
        writer.print("{}", .{n}) catch {};
        writer.flush() catch unreachable;
    }

    return buffer;
}

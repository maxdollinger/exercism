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

    if (writer.end == 0) {
        writer.print("{}", .{n}) catch {};
    }

    return writer.buffered();
}

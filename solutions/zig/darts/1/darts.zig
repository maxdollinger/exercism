const std = @import("std");

pub const Coordinate = struct {
    x: f32,
    y: f32,

    pub fn init(x_coord: f32, y_coord: f32) Coordinate {
        return .{ .x = x_coord, .y = y_coord };
    }

    pub fn score(self: Coordinate) usize {
        const dist = std.math.sqrt(self.x * self.x + self.y * self.y);

        if (dist <= 1) return 10;
        if (dist <= 5) return 5;
        if (dist <= 10) return 1;
        return 0;
    }
};

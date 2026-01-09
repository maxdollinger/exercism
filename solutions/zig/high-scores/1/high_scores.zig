const std = @import("std");

pub const HighScores = struct {
    scores: []const i32,

    pub fn init(scores: []const i32) HighScores {
        return .{
            .scores = scores,
        };
    }

    pub fn latest(self: *const HighScores) ?i32 {
        if (self.scores.len == 0) return null;
        return self.scores[self.scores.len - 1];
    }

    pub fn personalBest(self: *const HighScores) ?i32 {
        if (self.scores.len == 0) return null;

        var max: i32 = 0;
        for (self.scores) |pts| {
            max = @max(max, pts);
        }
        return max;
    }

    pub fn personalTopThree(self: *const HighScores) []const i32 {
        var top_three: [3]i32 = .{0} ** 3;

        for (self.scores) |score| {
            if (score > top_three[0]) {
                top_three[2] = top_three[1];
                top_three[1] = top_three[0];
                top_three[0] = score;
            } else if (score > top_three[1]) {
                top_three[2] = top_three[1];
                top_three[1] = score;
            } else if (score > top_three[2]) {
                top_three[2] = score;
            }
        }

        const result = top_three[0..@min(3, self.scores.len)];

        var gpa = std.heap.GeneralPurposeAllocator(.{}){};
        const allocator = gpa.allocator();
        const slice_copy = allocator.dupe(i32, result) catch return &[_]i32{};

        return slice_copy;
    }
};

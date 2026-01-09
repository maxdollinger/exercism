const std = @import("std");

pub const HighScores = struct {
    scores: []const i32,
    top_three: [3]i32,

    pub fn init(scores: []const i32) HighScores {
        var hs = HighScores{
            .scores = scores,
            .top_three = .{0} ** 3,
        };

        for (scores) |score| {
            if (score > hs.top_three[0]) {
                hs.top_three[2] = hs.top_three[1];
                hs.top_three[1] = hs.top_three[0];
                hs.top_three[0] = score;
            } else if (score > hs.top_three[1]) {
                hs.top_three[2] = hs.top_three[1];
                hs.top_three[1] = score;
            } else if (score > hs.top_three[2]) {
                hs.top_three[2] = score;
            }
        }

        return hs;
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
        return self.top_three[0..@min(3, self.scores.len)];
    }
};

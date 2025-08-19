pub fn isIsogram(str: []const u8) bool {
    var seen: u26 = 0;

    for (str) |char| {
        const offset = switch (char) {
            'a'...'z' => char - 'a',
            'A'...'Z' => char - 'A',
            else => continue,
        };

        const candidate = @as(u26, 1) << @as(u5, @truncate(offset));
        if (candidate & seen == candidate) {
            return false;
        }

        seen |= candidate;
    }

    return true;
}

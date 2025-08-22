pub fn isPangram(str: []const u8) bool {
    var seen: u26 = 0;

    for (str) |c| {
        const offset = switch (c) {
            'a'...'z' => c - 'a',
            'A'...'Z' => c - 'A',
            else => continue,
        };

        seen |= @as(u26, 1) << @as(u5, @truncate(offset));
        if (seen == 0b11111111111111111111111111) {
            return true;
        }
    }

    return false;
}

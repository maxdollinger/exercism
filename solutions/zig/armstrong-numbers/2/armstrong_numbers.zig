const std = @import("std");

pub fn isArmstrongNumber(num: u128) bool {
    if (num == 0) return true;

    const digit_count: usize = @intCast(std.math.log10(num)) + @as(usize, 1);

    var sum: u128 = 0;
    for (0..digit_count) |pos| {
        const digit = (num / std.math.pow(u128, 10, pos)) % 10;
        sum += std.math.pow(u128, digit, digit_count);
    }

    return sum == num;
}

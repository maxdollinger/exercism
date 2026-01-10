const std = @import("std");

pub fn squareRoot(n: u32) usize {
    if (n < 2) return n;

    var guess: usize = @as(usize, 1) << (@bitSizeOf(u32) - @clz(n)) / 2;

    guess = (guess + n / guess) / 2;

    return guess;
}

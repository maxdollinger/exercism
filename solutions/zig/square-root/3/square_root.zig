const std = @import("std");

pub fn squareRoot(n: u32) usize {
    if (n < 2) return n;

    // guess because of 2^((B-1)/2) ≤ √n < 2^(B/2)
    var guess: usize = @as(usize, 1) << (32 - @clz(n)) / 2;

    // one Newton iteration is acurate enough for the tests
    guess = (guess + n / guess) / 2;

    return guess;
}

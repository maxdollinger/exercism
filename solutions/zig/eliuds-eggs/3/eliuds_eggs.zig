const std = @import("std");

pub fn eggCount(number: usize) usize {
    return if (number == 0) 0 else (number & 1) + eggCount(number >> 1);
}

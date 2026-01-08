const std = @import("std");

pub fn eggCount(number: usize) usize {
    var tmp = number;
    var cnt: usize = 0;
    while (tmp > 0) : (tmp >>= 1) cnt += tmp & 1;

    return cnt;
}

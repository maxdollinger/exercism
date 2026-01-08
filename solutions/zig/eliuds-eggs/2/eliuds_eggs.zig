const std = @import("std");

pub fn eggCount(number: usize) usize {
    var tmp = number;
    var cnt: usize = 0;
    while (tmp > 0) : (tmp >>= 1) {
        if (tmp & 1 != 0) cnt += 1;
    }

    return cnt;
}

pub const ChessboardError = error{
    IndexOutOfBounds,
};

pub fn square(index: usize) ChessboardError!u64 {
    if (index > 64 or index < 1) return ChessboardError.IndexOutOfBounds;

    return @as(u64, 1) << @truncate(index - 1);
}

pub fn total() u64 {
    var sum: u64 = 1;
    while (sum >> 63 & 1 == 0) : (sum |= 1) {
        sum <<= 1;
    }

    return sum;
}

fn binarySearchImpl(comptime T: type, target: T, items: []const T, offset: usize) ?usize {
    if (items.len == 0) return null;
    if (target < items[0] or target > items[items.len - 1]) return null;

    const i = items.len / 2;
    const el = items[i];
    if (target < el) return binarySearchImpl(T, target, items[0..i], offset);
    if (target > el) return binarySearchImpl(T, target, items[i..], offset + i);

    return offset + i;
}

pub fn binarySearch(comptime T: type, target: T, items: []const T) ?usize {
    comptime {
        switch (@typeInfo(T)) {
            .int, .float, .comptime_int, .comptime_float => {},
            else => @compileError("Expected numeric type, got " ++ @typeName(T)),
        }
    }

    return binarySearchImpl(T, target, items, 0);
}

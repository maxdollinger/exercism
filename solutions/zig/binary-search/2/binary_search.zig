pub fn binarySearch(comptime T: type, target: T, items: []const T) ?usize {
    comptime {
        switch (@typeInfo(T)) {
            .int, .float, .comptime_int, .comptime_float => {},
            else => @compileError("Expected numeric type, got " ++ @typeName(T)),
        }
    }

    var lower_bound: usize = 0;
    var upper_bound = items.len;

    while (lower_bound < upper_bound) {
        const mid = lower_bound + (upper_bound - lower_bound) / 2;
        const item = items[mid];
        if (item == target) {
            return mid;
        } else if (item < target) {
            lower_bound = mid + 1;
        } else {
            upper_bound = mid;
        }
    }

    return null;
}

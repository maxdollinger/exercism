const std = @import("std");
const EnumSet = std.EnumSet;

pub const Allergen = enum(u4) {
    eggs,
    peanuts,
    shellfish,
    strawberries,
    tomatoes,
    chocolate,
    pollen,
    cats,
};

pub fn isAllergicTo(score: usize, allergen: Allergen) bool {
    return score & @as(usize, 1) << @intFromEnum(allergen) != 0;
}

pub fn initAllergenSet(score: usize) EnumSet(Allergen) {
    return .{ .bits = .{ .mask = @truncate(score) } };
}

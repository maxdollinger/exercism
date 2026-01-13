const std = @import("std");

pub fn modifier(score: i8) i8 {
    return @divFloor(score - 10, 2);
}

pub fn ability() i8 {
    const now = std.time.Instant.now() catch unreachable;
    const seed = @as(u64, @intCast(now.timestamp.nsec));
    var prng = std.Random.DefaultPrng.init(seed);
    const random = prng.random();

    var rolls = [_]i8{0} ** 4;
    for (0..rolls.len) |i| {
        rolls[i] = random.intRangeAtMost(i8, 1, 6);
    }
    std.sort.insertion(i8, &rolls, {}, std.sort.desc(i8));

    var sum: i8 = 0;
    for (0..3) |i| {
        sum += rolls[i];
    }

    return sum;
}

pub const Character = struct {
    strength: i8,
    dexterity: i8,
    constitution: i8,
    intelligence: i8,
    wisdom: i8,
    charisma: i8,
    hitpoints: i8,

    pub fn init() Character {
        var character = Character{
            .strength = ability(),
            .dexterity = ability(),
            .constitution = ability(),
            .intelligence = ability(),
            .wisdom = ability(),
            .charisma = ability(),
            .hitpoints = 0,
        };

        character.hitpoints = 10 + modifier(character.constitution);

        return character;
    }
};

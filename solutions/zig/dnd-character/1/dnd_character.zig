const std = @import("std");

pub fn modifier(score: i8) i8 {
    return @divFloor(score - 10, 2);
}

pub fn ability() i8 {
    var now = std.time.Instant.now() catch unreachable;
    const seed = @as(u64, @intCast(now.timestamp.nsec));
    var prng = std.Random.DefaultPrng.init(seed);
    const random = prng.random();

    var rolls = [_]i8{0} ** 3;
    for (0..3) |_| {
        const dice = random.intRangeAtMost(i8, 1, 6);

        if (dice > rolls[2]) {
            rolls[1] = rolls[2];
            rolls[0] = rolls[1];
            rolls[2] = dice;
        } else if (dice > rolls[1]) {
            rolls[0] = rolls[1];
            rolls[1] = dice;
        } else if (dice > rolls[0]) {
            rolls[0] = dice;
        }
    }

    var sum: i8 = 0;
    for (rolls) |value| {
        sum += value;
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

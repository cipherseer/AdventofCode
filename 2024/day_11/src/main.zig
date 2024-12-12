const std = @import("std");
const ndarray = @import("ndarray");

const input = @embedFile("input.txt");

const Stones = std.AutoHashMap(u64, u64);

//I tried a lookup table method but it was slower than log10_int
inline fn even_digits(number: u64) ?u64 {
    const digits = std.math.log10_int(number) + 1;
    return if (digits % 2 == 0) digits / 2 else null;
}

fn blink(stone: u64) struct { u64, ?u64 } {
    if (stone == 0) {
        return .{ 1, null };
    } else if (even_digits(stone)) |digits| {
        const divisor: u64 = std.math.pow(u64, 10, digits);
        return .{ stone / divisor, stone % divisor };
    } else {
        return .{ stone * 2024, null };
    }
}

//optimized solution ~4 ms
fn solve(stones: *Stones, new_stones: *Stones, steps: u8) u64 {
    for (0..steps) |_| {
        defer new_stones.clearRetainingCapacity();
        var entries = stones.iterator();
        while (entries.next()) |entry| {
            const k = entry.key_ptr.*;
            const v = entry.value_ptr.*;
            const a, const maybe_b = blink(k);
            var gop = new_stones.getOrPutAssumeCapacity(a);
            if (!gop.found_existing) gop.value_ptr.* = 0;
            gop.value_ptr.* += v;

            if (maybe_b) |b| {
                gop = new_stones.getOrPutAssumeCapacity(b);
                if (!gop.found_existing) gop.value_ptr.* = 0;
                gop.value_ptr.* += v;
            }
        }
        std.mem.swap(Stones, stones, new_stones);
    }

    var result: u64 = 0;
    var values = stones.valueIterator();
    while (values.next()) |v| result += v.*;
    return result;
}

//old solution ~17-18 ms
fn blink2(number: u64, steps: u8) !u64 {
    if (steps == 0) return 1;
    if (cache.get(.{ .n = number, .steps = steps })) |v| return v;

    var result: u64 = 0;

    if (number == 0) {
        result = try blink(1, steps - 1);
        //early return here slightly improves performance
        try cache.put(.{ .n = number, .steps = steps }, result);
        return result;
    }

    if (even_digits(number)) |digits| {
        const divisor: u64 = std.math.pow(u64, 10, digits / 2);
        const a: u64 = number / divisor;
        const b: u64 = number % divisor;
        result = try blink(a, steps - 1) + try blink(b, steps - 1);
    } else {
        result = try blink(number * 2024, steps - 1);
    }

    try cache.put(.{ .n = number, .steps = steps }, result);
    return result;
}

const Stone = struct {
    n: u64,
    steps: u8,
};

var cache: std.AutoHashMap(Stone, u64) = undefined;

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();
    var stones_it = std.mem.tokenizeScalar(u8, input[0 .. input.len - 1], ' ');

    var starting_stones = Stones.init(allocator);
    try starting_stones.ensureTotalCapacity(10000);
    while (stones_it.next()) |stone| {
        try starting_stones.putNoClobber(try std.fmt.parseInt(u64, stone, 10), 1);
    }

    var starting_stones2 = try starting_stones.clone();
    try starting_stones2.ensureTotalCapacity(10000);
    var new_stones = Stones.init(allocator);
    try new_stones.ensureTotalCapacity(10000);

    try stdout.print("Part 1: {}\n", .{solve(&starting_stones, &new_stones, 25)});
    try stdout.print("Part 2: {}\n", .{solve(&starting_stones2, &new_stones, 75)});

    try bw.flush();
}

const std = @import("std");
const ndarray = @import("ndarray");

const input = @embedFile("input.txt");

fn digitCount(n: u64) u8 {
    if (n == 0) return 1;

    // Precomputed powers of 10 to avoid runtime calculations
    const powers: [20]u64 = [_]u64{
        1,              10,              100,               1_000,              10_000,              100_000,               1_000_000,              10_000_000,              100_000_000,               1_000_000_000,
        10_000_000_000, 100_000_000_000, 1_000_000_000_000, 10_000_000_000_000, 100_000_000_000_000, 1_000_000_000_000_000, 10_000_000_000_000_000, 100_000_000_000_000_000, 1_000_000_000_000_000_000, 10_000_000_000_000_000_000,
    };

    // Approximation based on leading zeros
    const estimated_digits = (64 - @as(u64, @clz(n))) * 1233 >> 12;

    // Refine exact count using powers of 10
    return switch (estimated_digits) {
        0 => 1,
        1 => if (n < powers[1]) 1 else 2,
        2 => if (n < powers[2]) 2 else 3,
        3 => if (n < powers[3]) 3 else 4,
        4 => if (n < powers[4]) 4 else 5,
        5 => if (n < powers[5]) 5 else 6,
        6 => if (n < powers[6]) 6 else 7,
        7 => if (n < powers[7]) 7 else 8,
        8 => if (n < powers[8]) 8 else 9,
        9 => if (n < powers[9]) 9 else 10,
        10 => if (n < powers[10]) 10 else 11,
        11 => if (n < powers[11]) 11 else 12,
        12 => if (n < powers[12]) 12 else 13,
        13 => if (n < powers[13]) 13 else 14,
        14 => if (n < powers[14]) 14 else 15,
        15 => if (n < powers[15]) 15 else 16,
        16 => if (n < powers[16]) 16 else 17,
        17 => if (n < powers[17]) 17 else 18,
        18 => if (n < powers[18]) 18 else 19,
        else => if (n < powers[19]) 19 else 20, // Maximum digits for u64
    };
}

fn blink(n: u64, steps: u8) !u64 {
    if (steps == 0) return 1;
    if (cache.contains(.{ .n = n, .steps = steps })) return cache.get(.{ .n = n, .steps = steps }).?;

    const digits = digitCount(n);
    var result: u64 = 0;

    if (n == 0) {
        result = try blink(1, steps - 1);
    } else if (digits % 2 == 0) {
        const divisor: u64 = std.math.pow(u64, 10, digits / 2);
        const a: u64 = n / divisor;
        const b: u64 = n % divisor;
        result = try blink(a, steps - 1) + try blink(b, steps - 1);
    } else {
        result = try blink(n * 2024, steps - 1);
    }

    try cache.put(.{ .n = n, .steps = steps }, result);
    return result;
}

const Stone = struct {
    n: u64,
    steps: u8,
};

var cache: std.AutoHashMap(Stone, u64) = undefined;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer {
        _ = gpa.deinit();
    }
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    cache = std.AutoHashMap(Stone, u64).init(allocator);
    defer cache.deinit();

    var numbers = std.mem.tokenizeScalar(u8, input[0 .. input.len - 1], ' ');

    var result: u64 = 0;
    var result2: u64 = 0;
    while (numbers.next()) |slice| {
        const number = try std.fmt.parseInt(u64, slice, 10);
        result += try blink(number, 25);
        result2 += try blink(number, 75);
    }

    try stdout.print("Part 1: {}\n", .{result});
    try stdout.print("Part 2: {}\n", .{result2});

    try bw.flush();
}

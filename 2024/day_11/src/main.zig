const std = @import("std");
const ndarray = @import("ndarray");

const input = @embedFile("input.txt");

//I tried a lookup table method but it was slower than log10_int
inline fn even_digits(number: u64) ?u64 {
    const digits = std.math.log10_int(number) + 1;
    return if (digits % 2 == 0) digits else null;
}

fn blink(number: u64, steps: u8) !u64 {
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

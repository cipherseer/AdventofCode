const std = @import("std");

const input = @embedFile("test.txt");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer _ = gpa.deinit();
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    var seen = std.AutoHashMap(u64, void).init(allocator);
    defer seen.deinit();
   
    var lines = std.mem.tokenizeAny(u8, input, ",-\n");
    var result: u64 = 0;
    var result2: u64 = 0;
    while (lines.next()) |line| {
        var digits = line.len;
        const start = try std.fmt.parseInt(u64, line, 10);
        const next = lines.next().?;
        const end_digits = next.len;
        const end = try std.fmt.parseInt(u64, next, 10);

        while (digits <= end_digits) : (digits += 1) {
            var k: u64 = digits/2;
            while (k >= 1): (k -= 1) {
                if (digits % k != 0) continue;
                const max_power = try std.math.powi(u64, 10, digits);
                const min_power = try std.math.powi(u64, 10, k);
                const shift = try std.math.powi(u64, 10, digits - k);
                const multiplier = (max_power - 1) / (min_power - 1);
                var low = @max(start / shift, min_power / 10);
                const high = @min(end / shift, min_power - 1);
                while (low <= high) : (low += 1) {
                    const value = low * multiplier;
                    if (!seen.contains(value) and value >= start and value <= end) {
                        try seen.put(value, {});
                        if (2*k == digits) result += value;
                        result2 += value;
                    }
                }
                
            }
        }
    }
    try stdout.print("part 1: {}, part 2: {}\n", .{result, result2});

    try stdout.flush();
}

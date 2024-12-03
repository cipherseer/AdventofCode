const std = @import("std");

const input = @embedFile("input.txt");

const Change = enum { Increasing, Decreasing, Invalid };

pub fn main() !void {
    var buffer: [80]u8 = undefined;
    var fba = std.heap.FixedBufferAllocator.init(&buffer);
    const allocator = fba.allocator();

    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    var safe_reports_part1: u32 = 0;
    var safe_reports_part2: u32 = 0;
    while (it.next()) |line| {
        var numbers = std.mem.splitScalar(u8, line, ' ');
        var prev_number = try std.fmt.parseInt(i32, numbers.first(), 10);

        var list = std.ArrayList(i32).init(allocator);
        defer fba.reset();
        try list.append(prev_number);

        var delta: i32 = 0;
        var change: Change = .Invalid;
        var violated: bool = false;
        var index: usize = 0;
        while (numbers.next()) |slice| {
            index += 1;
            const number = try std.fmt.parseInt(i32, slice, 10);
            try list.append(number);
            delta = number - prev_number;

            if (delta >= 1 and delta <= 3 and (change == .Increasing or index == 1)) {
                change = .Increasing;
            } else if (delta <= -1 and delta >= -3 and (change == .Decreasing or index == 1)) {
                change = .Decreasing;
            } else {
                violated = true;
            }

            prev_number = number;
        }

        if (!violated) {
            safe_reports_part1 += 1;
            safe_reports_part2 += 1;
        } else {
            for (0..list.items.len) |i| {
                var prev_value: ?i32 = null;
                change = .Invalid;
                index = 0;
                violated = false;
                for (0..list.items.len) |j| {
                    if (j == i) continue;

                    const current = list.items[j];

                    if (prev_value) |prev| {
                        delta = current - prev;

                        if (delta >= 1 and delta <= 3 and (change == .Increasing or index == 1)) {
                            change = .Increasing;
                        } else if (delta <= -1 and delta >= -3 and (change == .Decreasing or index == 1)) {
                            change = .Decreasing;
                        } else {
                            violated = true;
                            break;
                        }
                    }
                    prev_value = current;
                    index += 1;

                    try bw.flush();
                }
                if (!violated) {
                    safe_reports_part2 += 1;
                    try bw.flush();
                    break;
                }
            }
        }
    }

    try stdout.print("Part 1: {}\n", .{safe_reports_part1});
    try stdout.print("Part 2: {}\n", .{safe_reports_part2});
    try bw.flush();
}

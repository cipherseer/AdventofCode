const std = @import("std");

const input = @embedFile("input.txt");

const Entry = struct {
    number: i64,
    index: usize,
    concated: bool,

    fn init(number: i64, index: usize, concated: bool) Entry {
        return .{ .number = number, .index = index, .concated = concated };
    }
};

inline fn p10(x: i64) i64 {
    var p: i64 = 10;
    while (x >= p) : (p *= 10) {}
    return p;
}

pub fn main() !void {
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    var lines = std.mem.tokenizeScalar(u8, input, '\n');

    var bridge: [64]i64 = undefined;
    var entries: [256]Entry = undefined;
    var result1: i64 = 0;
    var result2: i64 = 0;
    while (lines.next()) |line| {
        var halves = std.mem.splitScalar(u8, line, ':');
        const target = try std.fmt.parseInt(i64, halves.first(), 10);
        var numbers = std.mem.tokenizeScalar(u8, halves.rest(), ' ');
        var j: usize = 0;
        while (numbers.next()) |number| : (j += 1) {
            bridge[j] = try std.fmt.parseInt(i64, number, 10);
        }

        var top: usize = 0;
        entries[top] = Entry.init(target, j - 1, false);
        top += 1;

        while (top > 0) {
            top -= 1;
            const entry = entries[top];
            if (entry.index == 0) {
                if (entry.number != bridge[0]) continue;
                if (!entry.concated) result1 += target;
                result2 += target;
                break;
            } else {
                const next_num = bridge[entry.index];
                const index = entry.index - 1;

                if (entry.number - next_num >= 0) {
                    entries[top] = Entry.init(entry.number - next_num, index, entry.concated);
                    top += 1;
                }
                if (@mod(entry.number, next_num) == 0) {
                    entries[top] = Entry.init(@divExact(entry.number, next_num), index, entry.concated);
                    top += 1;
                }
                if (@mod(entry.number - next_num, p10(next_num)) == 0) {
                    entries[top] = Entry.init(@divExact(entry.number - next_num, p10(next_num)), index, true);
                    top += 1;
                }
            }
        }
    }

    try stdout.print("Part 1: {}\n", .{result1});
    try stdout.print("Part 2: {}\n", .{result2});

    try bw.flush();
}

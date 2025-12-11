const std = @import("std");

const input = @embedFile("test.txt");

pub fn main() !void {
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    var part1: u64 = 0;
    while (lines.next()) |line| {
        var sequence: [2]u8 = undefined;
        @memcpy(sequence[0..], line[0..2]);
        for (1..line.len-1) |i| {
            const next = line[i..i+2];
            var update: ?usize = null;

            inline for (0..sequence.len) |j| {
                if (next[j] > sequence[j]) {
                    update = j;
                    break;
                }
            }
    
            if (update) |k| {
                @memcpy(sequence[k..], next[k..]);
            }

        }
        const max_joltage = (sequence[0]-'0')*10+sequence[1]-'0';
        part1 += max_joltage;
    }

    lines.reset();
    var part2: u64 = 0;

    while (lines.next()) |line| {
        var sequence: [12]u8 = undefined;
        @memcpy(sequence[0..], line[0..12]);

        for (1..line.len-11) |i| {
            const next = line[i..i+12];
            var update: ?usize = null;

            inline for (0..sequence.len) |j| {
                if (next[j] > sequence[j]) {
                    update = j;
                    break;
                }
            }
    
            if (update) |k| {
                @memcpy(sequence[k..], next[k..]);
            }
        }

        var max_joltage: u64 = 0;
        for (0..12) |j| {
            max_joltage = 10*max_joltage + sequence[j]-'0';
        }
        part2 += max_joltage;

    }

    try stdout.print("part 1: {} - part 2: {}\n", .{part1, part2});
    try stdout.flush();
}

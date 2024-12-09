const std = @import("std");

const input = @embedFile("input.txt");

const Coords = struct {
    xs: [4]i8 = .{0} ** 4,
    ys: [4]i8 = .{0} ** 4,
    size: u3 = 0,
};

const Coord = struct {
    x: i8,
    y: i8,
};

inline fn char_to_index(c: u8) ?usize {
    switch (c) {
        '0'...'9' => return c - '0',
        'a'...'z' => return c - 'a' + 10,
        'A'...'Z' => return c - 'A' + 10 + 26,
        else => return null,
    }
}

inline fn index_to_char(i: usize) ?u8 {
    switch (i) {
        0...9 => return @as(u8, @intCast(i)) + '0',
        10...35 => return @as(u8, @intCast(i)) + 'a' - 10,
        36...62 => return @as(u8, @intCast(i)) + 'A' - 10 - 26,
        else => return null,
    }
}

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();

    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    var lines = std.mem.tokenizeScalar(u8, input, '\n');

    const cols: usize = lines.peek().?.len;
    var rows: usize = 0;

    var frequencies: [62]Coords = [_]Coords{.{}} ** 62;

    while (lines.next()) |line| : (rows += 1) {
        for (0..line.len) |col| {
            const index: usize = char_to_index(line[col]) orelse continue;
            const size = frequencies[index].size;
            frequencies[index].ys[size] = @as(i8, @intCast(rows));
            frequencies[index].xs[size] = @as(i8, @intCast(col));
            frequencies[index].size += 1;
        }
    }

    var antinodes1 = std.AutoHashMapUnmanaged(Coord, void){};
    try antinodes1.ensureTotalCapacity(allocator, @intCast(@divTrunc(rows * cols, 10)));
    var antinodes2 = std.AutoHashMapUnmanaged(Coord, void){};
    try antinodes2.ensureTotalCapacity(allocator, @intCast(@divTrunc(rows * cols, 2)));

    for (frequencies) |frequency| {
        if (frequency.size == 0) continue;

        for (0..(frequency.size - 1)) |i| {
            for ((i + 1)..frequency.size) |j| {
                const delta_x = frequency.xs[i] - frequency.xs[j];
                const delta_y = frequency.ys[i] - frequency.ys[j];
                var x1 = frequency.xs[i];
                var y1 = frequency.ys[i];
                var x2 = frequency.xs[j];
                var y2 = frequency.ys[j];

                var iterations: u8 = 0;
                while (x1 >= 0 and x1 < cols and y1 >= 0 and y1 < rows) {
                    if (iterations == 1) antinodes1.putAssumeCapacity(.{ .x = x1, .y = y1 }, {});
                    antinodes2.putAssumeCapacity(.{ .x = x1, .y = y1 }, {});

                    iterations += 1;
                    x1 += delta_x;
                    y1 += delta_y;
                }

                iterations = 0;
                while (x2 >= 0 and x2 < cols and y2 >= 0 and y2 < rows) {
                    if (iterations == 1) antinodes1.putAssumeCapacity(.{ .x = x2, .y = y2 }, {});
                    antinodes2.putAssumeCapacity(.{ .x = x2, .y = y2 }, {});

                    iterations += 1;
                    x2 -= delta_x;
                    y2 -= delta_y;
                }
            }
        }
    }

    try stdout.print("Part 1: {}\n", .{antinodes1.count()});
    try stdout.print("Part 2: {}\n", .{antinodes2.count()});

    try bw.flush();
}

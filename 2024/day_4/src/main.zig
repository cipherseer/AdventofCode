const std = @import("std");
const ndarray = @import("ndarray");

const input = @embedFile("input.txt");

fn solution_part1(table: *ndarray.Matrix(u8)) u32 {
    const directions: [8][2]i32 = .{ .{ 0, 1 }, .{ 1, 1 }, .{ 1, 0 }, .{ 1, -1 }, .{ 0, -1 }, .{ -1, -1 }, .{ -1, 0 }, .{ -1, 1 } };

    const word = "XMAS";
    const rows: usize = table.shape[0];
    const cols: usize = table.shape[1];

    var count: u32 = 0;
    for (0..rows) |y| {
        for (0..cols) |x| {
            if (table.at(.{ y, x }) == 'X') {
                outer: for (directions) |direction| {
                    const dx = direction[1];
                    const dy = direction[0];
                    const max_x: i32 = @as(i32, @intCast(x)) + 3 * dx;
                    const max_y: i32 = @as(i32, @intCast(y)) + 3 * dy;
                    if (max_y < 0 or max_y >= rows or max_x < 0 or max_x >= cols) continue;

                    for (1..4) |step| {
                        const xi: usize = @intCast(@as(i32, @intCast(x)) + @as(i32, @intCast(step)) * dx);
                        const yi: usize = @intCast(@as(i32, @intCast(y)) + @as(i32, @intCast(step)) * dy);

                        if (table.at(.{ yi, xi }) != word[step]) continue :outer;
                    }

                    count += 1;
                }
            }
        }
    }
    return count;
}

fn solution_part2(table: *ndarray.Matrix(u8)) u32 {
    const rows: usize = table.shape[0];
    const cols: usize = table.shape[1];

    var count: u32 = 0;
    for (1..rows - 1) |y| {
        for (1..cols - 1) |x| {
            if (table.at(.{ y, x }) == 'A') {
                const top_left: u8 = table.at(.{ y - 1, x - 1 });
                const top_right: u8 = table.at(.{ y - 1, x + 1 });
                const bottom_right: u8 = table.at(.{ y + 1, x + 1 });
                const bottom_left: u8 = table.at(.{ y + 1, x - 1 });

                if (top_left == 'M' and top_right == 'M' and bottom_right == 'S' and bottom_left == 'S' or
                    top_left == 'S' and top_right == 'M' and bottom_right == 'M' and bottom_left == 'S' or
                    top_left == 'S' and top_right == 'S' and bottom_right == 'M' and bottom_left == 'M' or
                    top_left == 'M' and top_right == 'S' and bottom_right == 'S' and bottom_left == 'M')
                {
                    count += 1;
                }
            }
        }
    }

    return count;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer {
        _ = gpa.deinit();
    }
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    var it = std.mem.tokenizeScalar(u8, input, '\n');
    var rows: usize = 0;
    const cols: usize = it.peek().?.len;
    while (it.next()) |_| : (rows += 1) {}

    var table = try ndarray.Matrix(u8).init(.{ rows, cols }, allocator);
    defer table.deinit();

    it.reset();
    var row: usize = 0;
    while (it.next()) |line| : (row += 1) {
        for (line, 0..) |c, col| {
            table.setAt(.{ row, col }, c);
        }
    }

    try stdout.print("Part 1: {}\n", .{solution_part1(&table)});
    try stdout.print("Part 2: {}\n", .{solution_part2(&table)});

    try bw.flush();
}

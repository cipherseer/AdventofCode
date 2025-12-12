const std = @import("std");

const input = @embedFile("test.txt");

pub fn main() !void {
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;
    
    const total: isize = @intCast(input.len);
    const width: isize = comptime @intCast(std.mem.indexOf(u8, input, "\n").?);
    const height: isize = comptime @divExact(total, width+1);

    const directions = [_][2]isize{.{-1,-1},.{-1,0},.{-1,1},
                         .{0,-1},         .{0,1},
                         .{1,-1}, .{1,0}, .{1,1}};

    var grid: [height][width]u8 = undefined;
    var remove: [height*width][2]usize = undefined;

    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    var k: usize = 0;
    while (lines.next()) |line| : (k+=1) {
        @memcpy(grid[k][0..], line);
    }
    
    var part1: usize = 0;
    var part2: usize = 0;

    var first_iteration: bool = true;
    while (true) {
        var i: isize = 0;
        var removal_count: usize = 0;
        while (i < height): (i+=1) {
            var j: isize = 0;
            while (j < width): (j+=1) {
                if (grid[@intCast(i)][@intCast(j)] != '@') continue;
                var count: u8 = 0;
                for (directions) |direction| {
                    const h = i+direction[0];
                    const w = j+direction[1];
                    if (h < 0 or h >= height or w < 0 or w >= width) continue;
                    if (grid[@intCast(h)][@intCast(w)] == '@') count += 1;
                }
                if (count < 4) {
                    remove[removal_count] = .{@intCast(i), @intCast(j)};
                    removal_count += 1;
                } 
            }
        }

        if (removal_count == 0) break;

        for (remove[0..removal_count]) |index| {
            if (first_iteration) part1 += 1;
            part2 += 1;
            grid[index[0]][index[1]] = '.';
        }
        first_iteration = false;
    }
    try stdout.print("Part 1: {} - Part 2: {}\n", .{part1, part2});
    try stdout.flush();
}

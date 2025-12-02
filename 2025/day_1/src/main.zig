const std = @import("std");

const input = @embedFile("test");

pub fn main() !void {
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;
   
    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    
    var dial: i32 = 50;
    var answer: u32 = 0;
    var answer2: i32 = 0;
    while (lines.next()) |line| {
        var rotation = try std.fmt.parseInt(i32, line[1..], 10);
        var crossings: i32 = 0;
        if (line[0] == 'L') {
            rotation = -rotation;
            crossings = @divFloor(rotation, -100) + 
                @intFromBool(dial + @mod(rotation, -100) <= 0 and dial != 0);
        } else {
            crossings = @divFloor(rotation, 100) +
                @intFromBool(dial + @mod(rotation, 100) >= 100);
        }
        dial = @mod(dial + rotation, 100);
        if (dial == 0) answer += 1;
        answer2 += crossings;
    }

    try stdout.print("Part 1: {}\nPart 2: {}\n", .{answer, answer2});

    try stdout.flush();
}

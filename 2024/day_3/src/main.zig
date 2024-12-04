const std = @import("std");

const input = @embedFile("input.txt");

fn solution_part1() u32 {
    var instructions = std.mem.tokenizeSequence(u8, input, "mul(");
    var result: u32 = 0;
    while (instructions.next()) |instruction| {
        var numbers = std.mem.tokenizeScalar(u8, instruction, ',');
        const a = std.fmt.parseInt(u32, numbers.next().?, 10) catch continue;
        var bracket = std.mem.tokenizeScalar(u8, numbers.rest(), ')');
        const b = std.fmt.parseInt(u32, bracket.next().?, 10) catch continue;
        result += a * b;
    }
    return result;
}

fn solution_part2() u32 {
    var dos = std.mem.tokenizeSequence(u8, input, "do()");
    var result: u32 = 0;
    while (dos.next()) |do| {
        var donts = std.mem.tokenizeSequence(u8, do, "don't()");
        var instructions = std.mem.tokenizeSequence(u8, donts.next().?, "mul(");
        while (instructions.next()) |instruction| {
            var numbers = std.mem.tokenizeScalar(u8, instruction, ',');
            const a = std.fmt.parseInt(u32, numbers.next().?, 10) catch continue;
            var bracket = std.mem.tokenizeScalar(u8, numbers.rest(), ')');
            const b = std.fmt.parseInt(u32, bracket.next().?, 10) catch continue;
            result += a * b;
        }
    }
    return result;
}

pub fn main() !void {
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    try stdout.print("Part 1: {}\n", .{solution_part1()});
    try stdout.print("Part 2: {}\n", .{solution_part2()});

    try bw.flush();
}

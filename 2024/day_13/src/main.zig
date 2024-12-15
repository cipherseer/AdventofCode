const std = @import("std");

const input = @embedFile("input.txt");

pub fn main() !void {
    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();
    
    var blocks = std.mem.tokenizeSequence(u8, input, "\n\n");
    
    var result1: i64 = 0;
    var result2: i64 = 0;
    while (blocks.next()) |block| {
        var numbers = std.mem.tokenizeAny(u8, block, "Button AB:XY+,Prize=\n");
        const a: i64 = try std.fmt.parseInt(i64, numbers.next().?, 10);
        const c: i64 = try std.fmt.parseInt(i64, numbers.next().?, 10);
        const b: i64 = try std.fmt.parseInt(i64, numbers.next().?, 10);
        const d: i64 = try std.fmt.parseInt(i64, numbers.next().?, 10);
        const x1: i64 = try std.fmt.parseInt(i64, numbers.next().?, 10);
        const y1: i64 = try std.fmt.parseInt(i64, numbers.next().?, 10);

        const det: i64 = a*d-b*c;
        if (det != 0) {
            const a_det: i64 = x1*d-y1*b;
            const b_det: i64 = y1*a-x1*c;
            if (@mod(a_det, det) == 0 and @mod(b_det,det) == 0) {
                result1 += 3 * @divExact(a_det, det);
                result1 += @divExact(b_det, det);
            }
        } else continue;

        const x2: i64 = x1 + 10000000000000;
        const y2: i64 = y1 + 10000000000000;
        const a_det: i64 = x2*d-y2*b;
        const b_det: i64 = y2*a-x2*c;
        if (@mod(a_det, det) == 0 and @mod(b_det,det) == 0) {
            result2 += 3 * @divExact(a_det, det);
            result2 += @divExact(b_det, det);
        }

    }

    try stdout.print("Part 1: {}\n", .{result1});
    try stdout.print("Part 2: {}\n", .{result2});

    try bw.flush();
}

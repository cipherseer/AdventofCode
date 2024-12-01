const std = @import("std");

const input = @embedFile("input.txt");

pub fn main() !void {
    //setup allocator and stdout
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    //storage of input data
    var list1 = std.ArrayList(i32).init(allocator);
    defer list1.deinit();
    var list2 = std.ArrayList(i32).init(allocator);
    defer list2.deinit();

    //hashmap to track duplicates for part 2
    var duplicates = std.AutoHashMap(i32, u32).init(allocator);
    defer duplicates.deinit();

    //parse each line of input
    var it = std.mem.tokenizeScalar(u8, input, '\n');
    while (it.next()) |line| {
        var split = std.mem.splitSequence(u8, line, "   ");

        const first = try std.fmt.parseInt(i32, split.first(), 10);
        const second = try std.fmt.parseInt(i32, split.next().?, 10);

        try list1.append(first);
        try list2.append(second);
    }

    //part 1
    std.mem.sort(i32, list1.items, {}, comptime std.sort.asc(i32));
    std.mem.sort(i32, list2.items, {}, comptime std.sort.asc(i32));

    var total: u32 = 0;

    for (list1.items, list2.items) |a, b| {
        total += @abs(a - b);
    }
    try stdout.print("total part 1: {}\n", .{total});

    //part 2
    for (list1.items) |key| {
        try duplicates.put(key, 0);
    }

    for (list2.items) |value| {
        if (duplicates.get(value)) |current| {
            try duplicates.put(value, current + 1);
        }
    }
    var total2: i64 = 0;
    for (list1.items) |a| {
        total2 += @as(i64, (duplicates.get(a) orelse 0)) * @as(i64, a);
    }

    try stdout.print("total part 2: {}\n", .{total2});
    try bw.flush();
}

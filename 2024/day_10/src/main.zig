const std = @import("std");
const ndarray = @import("ndarray");

const input = @embedFile("input.txt");

const Position = struct {
    y: usize,
    x: usize,
};

fn find_pinnacles(map: *ndarray.Matrix(u8), height: u8, p: Position, count: *usize, rating: *usize, pinnacles: *std.AutoHashMap(Position, void)) !void {
    const rows = map.shape[0] - 1;
    const cols = map.shape[1] - 1;

    if (height == 9) {
        if (!pinnacles.contains(p)) {
            count.* += 1;
            try pinnacles.put(p, {});
        }
        rating.* += 1;
        return;
    }
    const h = height + 1;

    if (p.y > 0 and map.at(.{ p.y - 1, p.x }) == h) {
        try find_pinnacles(map, h, .{ .y = p.y - 1, .x = p.x }, count, rating, pinnacles);
    }

    if (p.y < rows and map.at(.{ p.y + 1, p.x }) == h) {
        try find_pinnacles(map, h, .{ .y = p.y + 1, .x = p.x }, count, rating, pinnacles);
    }

    if (p.x > 0 and map.at(.{ p.y, p.x - 1 }) == h) {
        try find_pinnacles(map, h, .{ .y = p.y, .x = p.x - 1 }, count, rating, pinnacles);
    }

    if (p.x < cols and map.at(.{ p.y, p.x + 1 }) == h) {
        try find_pinnacles(map, h, .{ .y = p.y, .x = p.x + 1 }, count, rating, pinnacles);
    }
}

//strangely slightly slower than recursive solution
fn find_pinnacles_iter(map: *ndarray.Matrix(u8), start: Position, stack: *std.ArrayList(Position), visited: *std.AutoHashMap(Position, void)) !struct { usize, usize } {
    const rows = map.shape[0];
    const cols = map.shape[1];

    var count: usize = 0;
    var rating: usize = 0;

    try stack.append(start);
    while (stack.popOrNull()) |p| {
        var height = map.at(.{ p.y, p.x });
        if (height == 9) {
            if (!visited.contains(p)) {
                count += 1;
                try visited.put(p, {});
            }
            rating += 1;
            continue;
        }
        height += 1;

        if (p.y > 0 and map.at(.{ p.y - 1, p.x }) == height) try stack.append(.{ .y = p.y - 1, .x = p.x });
        if (p.y < rows - 1 and map.at(.{ p.y + 1, p.x }) == height) try stack.append(.{ .y = p.y + 1, .x = p.x });
        if (p.x > 0 and map.at(.{ p.y, p.x - 1 }) == height) try stack.append(.{ .y = p.y, .x = p.x - 1 });
        if (p.x < cols - 1 and map.at(.{ p.y, p.x + 1 }) == height) try stack.append(.{ .y = p.y, .x = p.x + 1 });
    }

    return .{ count, rating };
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

    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    const cols: usize = lines.peek().?.len;
    var rows: usize = 0;
    while (lines.next()) |_| : (rows += 1) {}
    lines.reset();

    var map = try ndarray.Matrix(u8).init(.{ rows, cols }, allocator);
    defer map.deinit();
    var trailheads = std.ArrayList(Position).init(allocator);
    defer trailheads.deinit();
    rows = 0;
    while (lines.next()) |line| : (rows += 1) {
        for (0..cols) |col| {
            const height = line[col] - '0';
            map.setAt(.{ rows, col }, height);
            if (height == 0) try trailheads.append(.{ .y = rows, .x = col });
        }
    }

    var pinnacles = std.AutoHashMap(Position, void).init(allocator);
    defer pinnacles.deinit();
    var result: usize = 0;
    var result2: usize = 0;
    for (trailheads.items) |trailhead| {
        defer pinnacles.clearRetainingCapacity();
        var count: usize = 0;
        var rating: usize = 0;
        // std.debug.print("counting pinnacles connected to trailhead at {}-{}\n", .{ trailhead.y, trailhead.x });
        try find_pinnacles(&map, 0, trailhead, &count, &rating, &pinnacles);
        result += count;
        result2 += rating;
    }

    try stdout.print("Part 1: {}\n", .{result});
    try stdout.print("Part 2: {}\n", .{result2});
    try bw.flush();
}

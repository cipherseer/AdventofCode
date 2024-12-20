const std = @import("std");
const ndarray = @import("ndarray");
// const deque = @import("zig-deque");

const input = @embedFile("test.txt");

const Maze = ndarray.Matrix(Node);

const Tile = enum {
    Wall,
    Empty,
    End,
};

const Node = struct {
    tile: Tile,
    score: usize = std.math.maxInt(usize),

    pub fn format(self: Tile, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        switch (self.tile) {
            .Wall => try writer.print("#", .{}),
            .Empty => try writer.print(".", .{}),
            .End => try writer.print("E", .{}),
        }
    }
};

const Reindeer = struct {
    y: isize,
    x: isize,
    dir: Direction,

    pub fn turn(self: *Reindeer) struct {Direction, Direction} {
        switch (self.dir) {
            .East, .West => .{.North, .South},
            .North, .South => .{.East, .West},
        }
    }
};

const Direction = enum {
    East,
    West,
    North,
    South,
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();
    _ = stdout; 
    const width: usize = std.mem.indexOf(u8, input, "\n").?;
    const height: usize = input.len / (width + 1); 

    var maze = try Maze.initWithValue(.{height,width}, .{ .tile = .Empty }, allocator);
    defer maze.deinit();

    var reindeer: Reindeer = undefined;

    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    
    var row: usize = 0;
    while (lines.next()) |line| :(row += 1) {
        for (0..line.len) |col| {
            switch (line[col]) {
                '#' => maze.setAt(.{row,col}, .{ .tile = .Wall}),
                'E' => maze.setAt(.{row,col}, .{ .tile =  .End}),
                'S' => {
                    reindeer = .{.y = @intCast(row), .x = @intCast(col), .dir = .East};
                    maze.setAt(.{row,col}, .{ .tile = .Empty, .score = 0 });
                },
                else => {},
            }
        }
    }

    var queue = std.PriorityQueue(Node, void, compare_node).init(allocator, {});
    defer queue.deinit();

    var visited = std.AutoHashMap(Reindeer, void).init(allocator);
    defer visited.deinit();

    queue.add(reindeer);

    
    try bw.flush();
}


fn compare_node(context: void, a: Node, b: Node) std.math.Order {
    _ = context;
    if (a.score < b.score) {
        return .lt;
    } else if (a.score > b.score) {
        return .gt;
    } else {
        return .eq;
    }
}

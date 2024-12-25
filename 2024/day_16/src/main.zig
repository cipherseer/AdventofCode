const std = @import("std");
const ndarray = @import("ndarray");

const input = @embedFile("input.txt");

const Maze = ndarray.Matrix(Tile);

const Tile = enum {
    Wall,
    Empty,
    End,

    pub fn format(self: Tile, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        switch (self) {
            .Wall => try writer.print("#", .{}),
            .Empty => try writer.print(".", .{}),
            .End => try writer.print("E", .{}),
        }
    }
};

const Node = struct {
    pos: Position,
    score: usize,
    prev: ?Position,

    pub fn neighbours(self: *const Node, nodes: *[3]Node) void {
        switch (self.pos.dir) {
            .East => {
                nodes[0] = .{ .pos = .{ .y = self.pos.y, .x = self.pos.x + 1, .dir = .East }, .score = self.score + 1, .prev = self.pos };
                nodes[1] = .{ .pos = .{ .y = self.pos.y, .x = self.pos.x, .dir = .North }, .score = self.score + 1000, .prev = self.pos };
                nodes[2] = .{ .pos = .{ .y = self.pos.y, .x = self.pos.x, .dir = .South }, .score = self.score + 1000, .prev = self.pos };
            },
            .West => {
                nodes[0] = .{ .pos = .{ .y = self.pos.y, .x = self.pos.x - 1, .dir = .West }, .score = self.score + 1, .prev = self.pos };
                nodes[1] = .{ .pos = .{ .y = self.pos.y, .x = self.pos.x, .dir = .North }, .score = self.score + 1000, .prev = self.pos };
                nodes[2] = .{ .pos = .{ .y = self.pos.y, .x = self.pos.x, .dir = .South }, .score = self.score + 1000, .prev = self.pos };
            },
            .North => {
                nodes[0] = .{ .pos = .{ .y = self.pos.y - 1, .x = self.pos.x, .dir = .North }, .score = self.score + 1, .prev = self.pos };
                nodes[1] = .{ .pos = .{ .y = self.pos.y, .x = self.pos.x, .dir = .East }, .score = self.score + 1000, .prev = self.pos };
                nodes[2] = .{ .pos = .{ .y = self.pos.y, .x = self.pos.x, .dir = .West }, .score = self.score + 1000, .prev = self.pos };
            },
            .South => {
                nodes[0] = .{ .pos = .{ .y = self.pos.y + 1, .x = self.pos.x, .dir = .South }, .score = self.score + 1, .prev = self.pos };
                nodes[1] = .{ .pos = .{ .y = self.pos.y, .x = self.pos.x, .dir = .East }, .score = self.score + 1000, .prev = self.pos };
                nodes[2] = .{ .pos = .{ .y = self.pos.y, .x = self.pos.x, .dir = .West }, .score = self.score + 1000, .prev = self.pos };
            },
        }
    }
};

const Position = struct {
    y: usize,
    x: usize,
    dir: Direction,
};

const Coord = struct {
    y: usize,
    x: usize,
};

const Direction = enum {
    East,
    West,
    North,
    South,
};

pub const Parents = struct {
    allocator: std.mem.Allocator,
    map: std.AutoHashMap(Position, std.AutoHashMap(?Position, void)),

    pub fn init(allocator: std.mem.Allocator) Parents {
        const map = std.AutoHashMap(Position, std.AutoHashMap(?Position, void)).init(allocator);
        return Parents{ .allocator = allocator, .map = map };
    }

    pub fn add(self: *Parents, key: Position, value: ?Position) !void {
        var entry = try self.map.getOrPut(key);

        if (!entry.found_existing) {
            entry.value_ptr.* = std.AutoHashMap(?Position, void).init(self.allocator);
        }

        try entry.value_ptr.put(value, {});
    }

    pub fn contains(self: *Parents, key: Position, value: ?Position) bool {
        const entry_option = self.map.get(key);
        if (entry_option) |entry| {
            return entry.contains(value);
        } else {
            return false;
        }
    }

    pub fn print(self: *Parents) void {
        var entries = self.map.iterator();
        while (entries.next()) |entry| {
            std.debug.print("key: {}\n", .{entry.key_ptr.*});
            var values = entry.value_ptr.iterator();

            while (values.next()) |value| {
                std.debug.print("{?} ", .{value.key_ptr.*});
            }
            std.debug.print("\n", .{});
        }
    }

    pub fn deinit(self: *Parents) void {
        var it = self.map.iterator();
        while (it.next()) |entry| {
            entry.value_ptr.deinit();
        }

        self.map.deinit();
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    const width: usize = std.mem.indexOf(u8, input, "\n").?;
    const height: usize = input.len / (width + 1);

    var maze = try Maze.initWithValue(.{ height, width }, .Empty, allocator);
    defer maze.deinit();

    var starting_node: Node = undefined;
    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    var row: usize = 0;
    while (lines.next()) |line| : (row += 1) {
        for (0..line.len) |col| {
            switch (line[col]) {
                '#' => maze.setAt(.{ row, col }, .Wall),
                'E' => maze.setAt(.{ row, col }, .End),
                'S' => starting_node = .{ .pos = .{ .y = row, .x = col, .dir = .East }, .score = 0, .prev = null },
                else => {},
            }
        }
    }

    var pq = std.PriorityQueue(Node, void, compare_node).init(allocator, {});
    defer pq.deinit();

    var visited = std.AutoHashMap(Position, usize).init(allocator);
    defer visited.deinit();

    var end_states = std.AutoHashMap(Position, void).init(allocator);
    defer end_states.deinit();

    var parents = Parents.init(allocator);
    defer parents.deinit();

    try pq.add(starting_node);

    var min_score: ?usize = null;

    while (pq.removeOrNull()) |node| {
        if (min_score) |score| {
            if (node.score > score) continue;
        }

        if (node.score > visited.get(node.pos) orelse std.math.maxInt(usize)) continue;
        try visited.put(node.pos, node.score);

        if (maze.at(.{ node.pos.y, node.pos.x }) == .End) {
            if (min_score == null) {
                min_score = node.score;
            }
            try end_states.put(node.pos, {});
            try parents.add(node.pos, node.prev);
            continue;
        }

        try parents.add(node.pos, node.prev);

        var new_nodes: [3]Node = undefined;

        node.neighbours(&new_nodes);

        for (new_nodes) |new_node| {
            if (maze.at(.{ new_node.pos.y, new_node.pos.x }) == .Wall) continue;

            const current_score = visited.get(new_node.pos) orelse std.math.maxInt(usize);

            if (new_node.score > current_score) continue;

            try pq.add(new_node);
        }
    }

    var unique_tiles = std.AutoHashMap(Coord, void).init(allocator);
    defer unique_tiles.deinit();

    var stack = std.ArrayList(Position).init(allocator);
    defer stack.deinit();

    var ends = end_states.keyIterator();
    while (ends.next()) |end| {
        try stack.append(end.*);
    }

    while (stack.popOrNull()) |position| {
        try unique_tiles.put(.{ .y = position.y, .x = position.x }, {});
        const entry = parents.map.get(position) orelse continue;
        var parent_iter = entry.iterator();
        while (parent_iter.next()) |value| {
            const next_position = value.key_ptr.*;
            if (next_position) |pos| {
                try stack.append(pos);
            }
        }
    }

    try stdout.print("Part 1: {}\n", .{min_score.?});
    try stdout.print("Part 2: {}\n", .{unique_tiles.count()});

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

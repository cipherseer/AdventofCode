const std = @import("std");
const ndarray = @import("ndarray");

const input = @embedFile("input.txt");

const Direction = enum {
    Up,
    Down,
    Left,
    Right,
};

const Tile = enum {
    Empty,
    Barrier,
};

const Position = struct {
    row: usize,
    col: usize,
};

const Guard = struct {
    position: Position,
    direction: Direction,

    pub fn patrol(self: *Guard, tiles: *ndarray.Matrix(Tile), positions: *std.AutoHashMap(Position, void), previous_states: *std.AutoHashMap(Guard, void), first: bool) !bool {
        const rows = tiles.shape[0];
        const cols = tiles.shape[1];

        defer previous_states.clearRetainingCapacity();

        while (true) {
            if (first) {
                try positions.put(self.position, {});
            }
            if (previous_states.contains(self.*)) {
                return true;
            } else {
                try previous_states.put(self.*, {});
            }
            switch (self.direction) {
                .Left => {
                    if (self.position.col >= 1) {
                        switch (tiles.at(.{ self.position.row, self.position.col - 1 })) {
                            .Empty => self.position.col -= 1,
                            .Barrier => self.direction = .Up,
                        }
                    } else {
                        break;
                    }
                },
                .Right => {
                    if (self.position.col < cols - 1) {
                        switch (tiles.at(.{ self.position.row, self.position.col + 1 })) {
                            .Empty => self.position.col += 1,
                            .Barrier => self.direction = .Down,
                        }
                    } else {
                        break;
                    }
                },
                .Up => {
                    if (self.position.row >= 1) {
                        switch (tiles.at(.{ self.position.row - 1, self.position.col })) {
                            .Empty => self.position.row -= 1,
                            .Barrier => self.direction = .Right,
                        }
                    } else {
                        break;
                    }
                },
                .Down => {
                    if (self.position.row < rows - 1) {
                        switch (tiles.at(.{ self.position.row + 1, self.position.col })) {
                            .Empty => self.position.row += 1,
                            .Barrier => self.direction = .Left,
                        }
                    } else {
                        break;
                    }
                },
            }
        }

        return false;
    }
};

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

    var tiles = try ndarray.Matrix(Tile).initWithValue(.{ rows, cols }, .Empty, allocator);
    defer tiles.deinit();
    var guard: Guard = undefined;

    var row: usize = 0;
    while (lines.next()) |line| : (row += 1) {
        for (0..line.len) |col| {
            switch (line[col]) {
                '#' => tiles.setAt(.{ row, col }, .Barrier),
                '^' => guard = .{ .position = .{ .row = row, .col = col }, .direction = .Up },
                'v' => guard = .{ .position = .{ .row = row, .col = col }, .direction = .Down },
                '>' => guard = .{ .position = .{ .row = row, .col = col }, .direction = .Right },
                '<' => guard = .{ .position = .{ .row = row, .col = col }, .direction = .Left },
                else => continue,
            }
        }
    }

    const initial = guard;
    var positions = std.AutoHashMap(Position, void).init(allocator);
    defer positions.deinit();
    var previous_states = std.AutoHashMap(Guard, void).init(allocator);
    try previous_states.ensureTotalCapacity(4 * @as(u32, @intCast(rows)) * @as(u32, @intCast(cols)));
    defer previous_states.deinit();

    _ = try guard.patrol(&tiles, &positions, &previous_states, true);

    var loop_barriers: u32 = 0;
    var position_it = positions.iterator();
    while (position_it.next()) |entry| {
        const position_to_block = entry.key_ptr.*;
        if (position_to_block.col != initial.position.col or position_to_block.row != initial.position.row) {
            tiles.setAt(.{ position_to_block.row, position_to_block.col }, .Barrier);
            if (try guard.patrol(&tiles, &positions, &previous_states, false)) {
                loop_barriers += 1;
            }
            tiles.setAt(.{ position_to_block.row, position_to_block.col }, .Empty);
            guard = initial;
            try bw.flush();
        }
    }

    try stdout.print("Part 1: {}\n", .{positions.count()});
    try stdout.print("Part 2: {}\n", .{loop_barriers});

    try bw.flush();
}

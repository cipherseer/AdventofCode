const std = @import("std");
const ndarray = @import("ndarray");
const deque = @import("zig-deque");

const Grid = ndarray.Matrix(Tile);
const Grid2 = ndarray.Matrix(Tile2);

const Tile = enum {
    Box,
    Wall,
    Empty,

    pub fn format(self: Tile, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        switch (self) {
            .Box => try writer.print("O", .{}),
            .Wall => try writer.print("#", .{}),
            .Empty => try writer.print(".", .{}),
        }
    }
};

const Tile2 = enum {
    Wall,
    Left_Box,
    Right_Box,
    Empty,

    pub fn format(self: Tile2, comptime fmt: []const u8, options: std.fmt.FormatOptions, writer: anytype) !void {
        _ = fmt;
        _ = options;
        switch (self) {
            .Wall => try writer.print("#", .{}),
            .Left_Box => try writer.print("[", .{}),
            .Right_Box => try writer.print("]", .{}),
            .Empty => try writer.print(".", .{}),
        }
    }
};

const Direction = enum {
    Left,
    Right,
    Up,
    Down,

    pub fn dir_to_deltas(self: Direction) struct {i8,i8} {
        switch (self) {
            .Left => return .{0,-1},
            .Right => return .{0,1},
            .Up => return .{-1,0},
            .Down => return .{1,0},
        }
    }
};


const Position = struct {
    x: isize,
    y: isize,
};


fn move_box(warehouse: *Grid, robot: *Position, dir: Direction) void {
    const dy, const dx = dir.dir_to_deltas();

    const ny =robot.y + dy;
    const nx =robot.x + dx;
    
    var nny = ny + dy;
    var nnx = nx + dx;
    var tile: Tile = warehouse.at(.{@intCast(nny), @intCast(nnx)});

    while (tile == .Box) {
        nny += dy;
        nnx += dx;
        tile = warehouse.at(.{@intCast(nny), @intCast(nnx)});
    }
    
    if (tile == .Empty) {
        warehouse.setAt(.{@intCast(ny),@intCast(nx)}, .Empty);
        warehouse.setAt(.{@intCast(nny),@intCast(nnx)}, .Box);
        robot.y = ny;
        robot.x = nx;
    }
}

fn move_robot(warehouse: *Grid, robot: *Position, dir: Direction) void {
    const dy, const dx = dir.dir_to_deltas();

    const ny = robot.y + dy;
    const nx = robot.x + dx;

    switch (warehouse.at(.{@intCast(ny),@intCast(nx)})) {
        .Empty => {
            robot.y = ny;
            robot.x = nx;
        },
        .Box => move_box(warehouse, robot, dir),
        .Wall => {},
    }
}

fn move_boxes(warehouse: *Grid2, robot: *Position, dir: Direction, arena: *std.heap.ArenaAllocator) !void {    
    defer _ = arena.reset(.retain_capacity);
    const alloc = arena.allocator();
    const dy, const dx = dir.dir_to_deltas();
    var can_move: bool = true;

    var moveable_boxes = std.ArrayList(Position).init(alloc);
    var positions_to_check = try deque.Deque(Position).init(alloc);

    try positions_to_check.pushBack(.{.y = robot.y+dy, .x = robot.x+dx});
    
    while (positions_to_check.popFront()) |position| {
       switch (warehouse.at(.{@intCast(position.y), @intCast(position.x)})) {
            .Wall => {
                can_move = false;
                break;
            },
            .Empty => {},
            .Left_Box => {
                try moveable_boxes.append(.{ .y = position.y, .x = position.x });
                switch (dir) {
                    .Left => can_move = false, 
                    .Right => try positions_to_check.pushBack(.{ .y = position.y, .x = position.x + 2 }),
                    .Up, .Down => {
                        try positions_to_check.pushBack(.{.y = position.y + dy, .x = position.x});
                        try positions_to_check.pushBack(.{.y = position.y + dy, .x = position.x+1});
                    },
                }
            },
            .Right_Box => {
                //measure each box from left side by convention
                try moveable_boxes.append(.{ .y = position.y, .x = position.x - 1});
                switch (dir) {
                    .Left => try positions_to_check.pushBack(.{ .y = position.y, .x = position.x - 2 }),
                    .Right => can_move = false,
                    .Up, .Down => {
                        try positions_to_check.pushBack(.{.y = position.y + dy, .x = position.x});
                        try positions_to_check.pushBack(.{.y = position.y + dy, .x = position.x - 1});
                    }
                }
            },
        }
    }

    if (can_move) {
        robot.y += dy;
        robot.x += dx;
        
        while (moveable_boxes.popOrNull()) |box| {
            warehouse.setAt(.{@intCast(box.y),@intCast(box.x)}, .Empty);
            warehouse.setAt(.{@intCast(box.y), @intCast(box.x+1)}, .Empty);
            warehouse.setAt(.{@intCast(box.y+dy),@intCast(box.x+dx)}, .Left_Box);
            warehouse.setAt(.{@intCast(box.y+dy),@intCast(box.x+dx+1)}, .Right_Box);
        }
    }

}

fn move_robot2(warehouse: *Grid2, robot: *Position, dir: Direction, arena: *std.heap.ArenaAllocator) !void {
    const dy, const dx = dir.dir_to_deltas();

    const ny = robot.y + dy;
    const nx = robot.x + dx;

    switch (warehouse.at(.{@intCast(ny), @intCast(nx)})) {
        .Empty => {
            robot.y = ny;
            robot.x = nx;
        },
        .Left_Box, .Right_Box => try move_boxes(warehouse, robot, dir, arena),
        .Wall => {}
    }

}

const input = @embedFile("input.txt");

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();

    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    var blocks = std.mem.tokenizeSequence(u8, input, "\n\n");
    
    const width = std.mem.indexOf(u8, blocks.peek().?, "\n").?;
    const height = blocks.peek().?.len / width;
    
    var warehouse = try Grid.initWithValue(.{height,width}, Tile.Empty, allocator);
    defer warehouse.deinit();

    var warehouse2 = try Grid2.initWithValue(.{height, width*2}, Tile2.Empty, allocator);
    defer warehouse2.deinit();

    var robot: Position = undefined;
    var robot2: Position = undefined;
    var rows = std.mem.tokenizeScalar(u8, blocks.next().?, '\n');
    
    var k: usize = 0;
    while (rows.next()) |row| :(k+=1) {
        for (0..width) |j| {
            switch (row[j]) {
                '#' => {
                    warehouse.setAt(.{k,j}, .Wall);
                    warehouse2.setAt(.{k,2*j}, .Wall);
                    warehouse2.setAt(.{k,2*j+1}, .Wall);
                },
                'O' => {
                    warehouse.setAt(.{k,j}, .Box);
                    warehouse2.setAt(.{k,2*j}, .Left_Box);
                    warehouse2.setAt(.{k,2*j+1}, .Right_Box);
                },
                '@' => {
                    robot = .{ .x = @intCast(j), .y = @intCast(k)};
                    robot2 = .{ .x = @intCast(2*j), .y = @intCast(k) };
                },
                else => {}
            }
        }
    }


    var directions = std.ArrayList(Direction).init(allocator);
    defer directions.deinit();

    const instructions = blocks.next().?;

    for (instructions) |instruction| {
        switch (instruction) {
            '<' => try directions.append(.Left),
            '>' => try directions.append(.Right),
            '^' => try directions.append(.Up),
            'v' => try directions.append(.Down),
            else => {}
        }
    }
    
    for (directions.items) |direction| {
        move_robot(&warehouse, &robot, direction);
        try move_robot2(&warehouse2, &robot2, direction, &arena);
    }

    var result: usize = 0;

    for (0..height) |i| {
        for (0..width) |j| {
            if (warehouse.at(.{i,j}) == .Box) result += 100*i+j;
        }
    }
    
    var result2: usize = 0;
    for (0..height) |i| {
        for (0..width*2) |j| {
            if (warehouse2.at(.{i,j}) == .Left_Box) result2 += 100*i+j;
        }
    }


    try stdout.print("Part 1: {}\n", .{result});
    try stdout.print("Part 2: {}\n", .{result2});

    try bw.flush();
}

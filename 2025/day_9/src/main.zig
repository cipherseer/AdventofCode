const std = @import("std");

const input = @embedFile("test.txt");

const Position = struct {
    x: isize,
    y: isize,
};

const CompressedPosition = struct {
    x: usize,
    y: usize,
};

fn min_sqr_distance(_: void, a: Position, b: Position) bool {
    const sqr_dist_a = a.x*a.x+a.y*a.y;
    const sqr_dist_b = b.x*b.x+b.y*b.y;
    return sqr_dist_a < sqr_dist_b;
}

fn sort_unique(coords: *std.ArrayList(isize)) void {
    std.mem.sort(isize, coords.items, {}, std.sort.asc(isize));
    var write: usize = 0;
    //deduplicate in place
    for (coords.items) |coord| {
        if (write == 0 or coords.items[write - 1] != coord) {
            coords.items[write] = coord;
            write += 1;
        }
    }
    coords.items.len = write;
}

fn asc(a: isize, b: isize) std.math.Order {
    return std.math.order(a, b);
}

const Queue = struct {
    allocator: std.mem.Allocator,
    buffer: std.ArrayList(Position),
    head: usize,

    fn init(allocator: std.mem.Allocator) !Queue {
        const items = try std.ArrayList(Position).initCapacity(allocator, 10);
        return .{.allocator=allocator, .buffer=items, .head=0};
    }

    fn deinit(self: *Queue) void {
        self.buffer.deinit(self.allocator);
    }

    fn append(self: *Queue, item: Position) !void {
        try self.buffer.append(self.allocator, item);
    }

    fn pop(self: *Queue) ?Position {
        if (self.head >= self.buffer.items.len) return null;
        const val = self.buffer.items[self.head];
        self.head += 1;

        if (self.head >= 64) {
            const remaining_len = self.buffer.items.len - self.head;
            @memmove(self.buffer.items[0..remaining_len], self.buffer.items[self.head..]);
            self.buffer.items.len = remaining_len;
            self.buffer.items = self.buffer.items[0..remaining_len];
            self.head = 0;
        }

        return val;
    }

};

const Tile = enum {
    Outside,
    Inside,
    Boundary,
};

const CompressedGrid = struct {
    allocator: std.mem.Allocator,
    rows: std.ArrayList(isize),
    cols: std.ArrayList(isize),
    grid: [][]Tile,
    prefix_sum: [][]usize,
    
    const Self = @This();
    fn init(allocator: std.mem.Allocator, rows: std.ArrayList(isize), 
                                              cols: std.ArrayList(isize)) !Self {

        const grid = try allocator.alloc([]Tile, rows.items.len);
        errdefer allocator.free(grid);
        for (grid) |*row| {
            row.* = try allocator.alloc(Tile, cols.items.len);
            @memset(row.*, Tile.Inside);
        }

        const prefix_sum = try allocator.alloc([]usize, rows.items.len);
        errdefer allocator.free(prefix_sum);
        for (prefix_sum) |*row| {
            row.* = try allocator.alloc(usize, cols.items.len);
            @memset(row.*, 0);
        }

        return .{.allocator=allocator, .rows=rows, .cols=cols, .grid=grid, .prefix_sum=prefix_sum};
    }

    fn deinit(self: *Self) void {
        for (self.grid) |row| self.allocator.free(row);
        self.allocator.free(self.grid);

        for (self.prefix_sum) |row| self.allocator.free(row);
        self.allocator.free(self.prefix_sum);
    }

    inline fn cx(self: Self, x: isize) usize {
        return std.sort.binarySearch(isize, self.cols.items, x, asc).?;
    }


    inline fn cy(self: Self, y: isize) usize {
        return std.sort.binarySearch(isize, self.rows.items, y, asc).?;
    }

    fn compress_positions(self: Self, positions: []Position) !std.ArrayList(CompressedPosition) {
        var compressed = try std.ArrayList(CompressedPosition).initCapacity(self.allocator, positions.len);

        for (positions) |p| {
            compressed.appendAssumeCapacity(.{.y=self.cy(p.y), .x=self.cx(p.x)});
        }

        return compressed;
    }

    fn draw_boundary(self: *Self, positions: []CompressedPosition) void {
        const max_index = positions.len;
        var prev_x = positions[max_index-1].x;
        var prev_y = positions[max_index-1].y;
        for (0..positions.len) |i| {
            const next_x = positions[i].x; 
            const next_y = positions[i].y; 
            if (next_y == prev_y) {
                const x0 = @min(prev_x, next_x);
                const x1 = @max(prev_x, next_x)+1;
                for (x0..x1) |x| self.grid[next_y][x] = .Boundary; 
            } else {
                const y0 = @min(prev_y, next_y);
                const y1 = @max(prev_y, next_y)+1;
                for (y0..y1) |y| self.grid[y][next_x] = .Boundary;
            }
            prev_x = next_x;
            prev_y = next_y;
        }
    }

    fn flood_fill(self: *Self) !void {
        var queue = try Queue.init(self.allocator);
        defer queue.deinit();
        try queue.append(.{.x=0,.y=0});
        self.grid[0][0] = .Outside;

        const directions = [_][2]isize{.{-1,0},.{0,-1},.{0,1},.{1,0}};
        while (queue.pop()) |pos| {
           for (directions) |dir| {
                const ny = pos.y + dir[0];
                const nx = pos.x + dir[1];
                if (ny < 0 or ny >= self.rows.items.len or 
                    nx < 0 or nx >= self.cols.items.len or 
                    self.grid[@intCast(ny)][@intCast(nx)] != .Inside) continue;
                self.grid[@intCast(ny)][@intCast(nx)] = .Outside;
                try queue.append(.{.y=ny,.x=nx});
            } 
        }
    }

    fn construct_prefix_sum(self: *Self) void {
        for (1..self.rows.items.len) |y| {
            for (1..self.cols.items.len) |x| {
                const left = self.prefix_sum[y][x-1];
                const top = self.prefix_sum[y-1][x];
                const top_left = self.prefix_sum[y-1][x-1];
                var value: usize = 0;
                switch (self.grid[y][x]) {
                    .Outside => value = 1,
                    .Inside, .Boundary => value = 0,
                }
                self.prefix_sum[y][x] = left + top - top_left + value; 
            }
        }
    }


    fn valid_rectangle(self: Self, a: CompressedPosition, b: CompressedPosition) bool {
        const x1 = @min(a.x, b.x);
        const y1 = @min(a.y, b.y);
        const x2 = @max(a.x, b.x);
        const y2 = @max(a.y, b.y);
        const left = self.prefix_sum[y2][x1-1];
        const top = self.prefix_sum[y1-1][x2];
        const top_left = self.prefix_sum[y1-1][x1-1];
        const count = self.prefix_sum[y2][x2] + top_left - left - top;

        return count == 0;
    }
    
};

inline fn area(a: Position, b: Position) usize {
    const dx = @abs(a.x-b.x) + 1;
    const dy = @abs(a.y-b.y) + 1;
    return dx * dy;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    var tokens = std.mem.tokenizeAny(u8, input, "\n,");
    var positions = try std.ArrayList(Position).initCapacity(allocator, 8);
    defer positions.deinit(allocator);
    var cols = try std.ArrayList(isize).initCapacity(allocator, positions.items.len);
    defer cols.deinit(allocator);
    var rows = try std.ArrayList(isize).initCapacity(allocator, positions.items.len);
    defer rows.deinit(allocator);
    
    //Part 1
    //sort positions by squared distance to top left corner
    //taking the first and last elements should maximize area
    var left_positions = try std.ArrayList(Position).initCapacity(allocator, 8);
    defer left_positions.deinit(allocator);
    
    var min_col: isize = std.math.maxInt(isize);
    var max_col: isize = std.math.minInt(isize);
    var min_row: isize = std.math.maxInt(isize);
    var max_row: isize = std.math.minInt(isize);
    while (tokens.index < tokens.buffer.len - 1) {
        const x: isize = try std.fmt.parseInt(isize, tokens.next().?, 10);
        const y: isize = try std.fmt.parseInt(isize, tokens.next().?, 10);
        try positions.append(allocator, .{.x=x,.y=y});
        try cols.append(allocator, x);
        try rows.append(allocator, y);
        if (x < min_col) min_col = x;
        if (x > max_col) max_col = x;
        if (y < min_row) min_row = y;
        if (y > max_row) max_row = y;
    }
    try left_positions.ensureTotalCapacityPrecise(allocator, positions.items.len);
    left_positions.items.len = positions.items.len;
    @memcpy(left_positions.items, positions.items);

    std.mem.sort(Position, left_positions.items, {}, min_sqr_distance);
    
    const max_index: usize = left_positions.items.len-1;
    const part1 = area(left_positions.items[0], left_positions.items[max_index]);

    //Part 2
    //pad by one tile to allow flood fill
    try cols.append(allocator, min_col-1);
    try cols.append(allocator, max_col+1);
    try rows.append(allocator, min_row-1);
    try rows.append(allocator, max_row+1);
    sort_unique(&cols);
    sort_unique(&rows);
    
    var cg = try CompressedGrid.init(allocator, rows, cols);
    defer cg.deinit();
    var compressed_positions = try cg.compress_positions(positions.items);
    defer compressed_positions.deinit(allocator);
    cg.draw_boundary(compressed_positions.items);                 
    try cg.flood_fill();   
    cg.construct_prefix_sum();
    
    var part2: usize = 0;
    for (0..positions.items.len-1) |i| {
        for (i..positions.items.len) |j| {
            if (cg.valid_rectangle(compressed_positions.items[i], compressed_positions.items[j])) {
                const a = area(positions.items[i], positions.items[j]);
                if (part2 < a) part2 = a;
            }
        }
    }

    try stdout.print("Part 1: {}, Part 2: {}\n", .{part1, part2});
    
    try stdout.flush();
}

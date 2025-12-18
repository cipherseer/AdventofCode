const std = @import("std");

const input = @embedFile("test.txt");

const Coords = struct {
    x: isize,
    y: isize,
    z: isize,
};

const CoordPair = struct {
    a_index: usize,
    b_index: usize,
};

fn squared_distance(c: CoordPair, coords: []const Coords) isize {
    const a = coords[c.a_index];
    const b = coords[c.b_index];
    const dx = a.x-b.x;
    const dy = a.y-b.y;
    const dz = a.z-b.z;
    return dx*dx+dy*dy+dz*dz;
}

fn min_squared_distance(coords: []const Coords, c1: CoordPair, c2: CoordPair) std.math.Order {
    return std.math.order(squared_distance(c1,coords), squared_distance(c2,coords));
}

const DisjointSet = struct {
    parent: std.ArrayList(usize),
    size: std.ArrayList(usize),
    allocator: std.mem.Allocator,
    
    const Self = @This();
    pub fn init(allocator: std.mem.Allocator, n: usize) !DisjointSet {
        var parent = try std.ArrayList(usize).initCapacity(allocator, n);
        var size = try std.ArrayList(usize).initCapacity(allocator, n);
        for (0..n) |i| {
            try parent.append(allocator, i);
            try size.append(allocator, 1);
        }
        return .{.parent=parent, .size=size, .allocator=allocator};
    }

    pub fn deinit(self: *Self) void {
        self.parent.deinit(self.allocator);
        self.size.deinit(self.allocator);
    }

    pub fn find(self: *Self, x: usize) usize {
        if (self.parent.items[x] != x) {
            self.parent.items[x] = self.find(self.parent.items[x]);
        }
        return self.parent.items[x];
    }

    pub fn join(self: *Self, x: usize, y: usize) usize {
        const x_parent = self.find(x);
        const y_parent = self.find(y);

        if (x_parent == y_parent) return self.size.items[x_parent];

        const x_size = self.size.items[x_parent];
        const y_size = self.size.items[y_parent];

        if (x_size < y_size) {
            self.parent.items[x_parent] = y_parent;
            self.size.items[y_parent] += self.size.items[x_parent];
            return self.size.items[y_parent];
        } else {
            self.parent.items[y_parent] = x_parent;
            self.size.items[x_parent] += self.size.items[y_parent];
            return self.size.items[x_parent];
        }
    }
};

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{.safety = false}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    var tokens = std.mem.tokenizeAny(u8, input, "\n,");
    var coords = try std.ArrayList(Coords).initCapacity(allocator, 20);
    defer coords.deinit(allocator);
    
    var num_points: usize = 0;
    while (tokens.index < tokens.buffer.len - 1) : (num_points+=1) {
        const x: isize = try std.fmt.parseInt(isize, tokens.next().?, 10);
        const y: isize = try std.fmt.parseInt(isize, tokens.next().?, 10);
        const z: isize = try std.fmt.parseInt(isize, tokens.next().?, 10);
        try coords.append(allocator, .{.x=x,.y=y,.z=z});
    }
    
    var edge_queue = std.PriorityQueue(CoordPair,[]const Coords, min_squared_distance)
                                    .init(allocator, coords.items);
    defer edge_queue.deinit();


    for (0..coords.items.len-1) |i| {
        for (i+1..coords.items.len) |j| {
            try edge_queue.add(.{.a_index=i,.b_index=j});
        }
    }

    var disjoint_set = try DisjointSet.init(allocator, num_points);
    defer disjoint_set.deinit();
    
    var k: usize = 0;
    while (edge_queue.peek()) |pair| : (k+=1) {
        if (k == 1000) break;
        _ = edge_queue.remove();
        _ = disjoint_set.join(pair.a_index, pair.b_index);
    }
    
    var max_sizes: [3]usize = .{0,0,0};
    for (disjoint_set.size.items) |size| {
        if (size > max_sizes[0]) {
            max_sizes[2] = max_sizes[1];
            max_sizes[1] = max_sizes[0];
            max_sizes[0] = size;
        } else if (size > max_sizes[1]) {
            max_sizes[2] = max_sizes[1];
            max_sizes[1] = size;
        } else if (size > max_sizes[0]) {
            max_sizes[0] = size;
        }
    }
    var part1: usize = 1;
    for (max_sizes) |s| {
        part1 *= s;
    }
    
    var part2: isize = 0;
    while (edge_queue.peek()) |pair| {
        _ = edge_queue.remove();
        const size = disjoint_set.join(pair.a_index, pair.b_index);

        if (size == num_points) {
            part2 = coords.items[pair.a_index].x * coords.items[pair.b_index].x;
            break;
        }
    }

    try stdout.print("Part 1: {} - Part 2: {}\n", .{part1, part2});

    try stdout.flush();
}

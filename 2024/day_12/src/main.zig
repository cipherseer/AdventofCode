const std = @import("std");
const ndarray = @import("ndarray");

const input = @embedFile("input.txt");

const Region = struct {
    id: u8,
    area: u32,
    perimeter: u32,
    sides: u32,
};

const Position = struct {
    y: i32,
    x: i32,
};

const RegionMap = std.AutoHashMap(Position, *Region);
const Garden = ndarray.Matrix(u8);

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    const cols: usize = std.mem.indexOf(u8, input, "\n").?;
    const rows: usize = @divTrunc(input.len, cols + 1);

    var garden = try ndarray.Matrix(u8).init(.{ rows, cols }, allocator);
    defer garden.deinit();

    var row: usize = 0;
    while (lines.next()) |line| : (row += 1) {
        for (0..line.len) |col| {
            garden.setAt(.{ row, col }, line[col]);
        }
    }

    var region_map = RegionMap.init(allocator);
    defer region_map.deinit();

    var regions = std.ArrayList(Region).init(allocator);
    defer regions.deinit();

    for (0..rows) |i| {
        for (0..cols) |j| {
            const key: Position = .{ .y = @intCast(i), .x = @intCast(j)};
            if (region_map.contains(key)) continue;
            try regions.append(.{.id = garden.at(.{i,j}), .area = 0, .perimeter = 0, .sides = 0});

            try flood_fill(key.y, key.x, &garden, &regions.items[regions.items.len - 1], &region_map);
        }
    }
    
    var result1: usize = 0;
    var result2: usize = 0;
    for (regions.items) |region| {
        result1 += region.area * region.perimeter;
        result2 += region.area * region.sides;
    }
    try stdout.print("Part 1: {}\n", .{result1});
    try stdout.print("Part 2: {}\n", .{result2});

    try bw.flush();
}


fn valid_coord(y: i32, x: i32, garden: *Garden, id: u8) bool {
    const rows = garden.shape[0];
    const cols = garden.shape[1];

    if (y >= 0 and y < rows and x >= 0 and x < cols 
        and garden.at(.{@intCast(y),@intCast(x)}) == id) return true;

    return false;
}

fn flood_fill(y: i32, x: i32, garden: *Garden, region: *Region, map: *RegionMap) !void {
    const key: Position = .{.y = y, .x = x};
    if (map.contains(key)) return;
    try map.put(key, region);
    const id = garden.at(.{@intCast(y), @intCast(x)});
    
    const right: bool = valid_coord(y, x+1, garden, id);
    const left: bool =  valid_coord(y, x-1, garden, id); 
    const up: bool =    valid_coord(y-1, x, garden, id);   
    const down: bool =  valid_coord(y+1, x, garden, id);
    
    if (right) {
        try flood_fill(y, x+1, garden, region, map);
    } else {
        region.perimeter += 1;
    }

    if (left) {
        try flood_fill(y, x-1, garden, region, map);
    } else {
        region.perimeter += 1;
    }

    if (up) {
        try flood_fill(y-1, x, garden, region, map);
    } else {
        region.perimeter += 1;
    }

    if (down) {
        try flood_fill(y+1, x, garden, region, map);
    } else {
        region.perimeter += 1;
    }

    //Count sides
    if (!down and !left)                                      region.sides += 1;
    if (!down and !right)                                     region.sides += 1; 
    if (!up   and !left )                                     region.sides += 1; 
    if (!up   and !right)                                     region.sides += 1; 
    if (up    and  right and !valid_coord(y-1,x+1,garden,id)) region.sides += 1; 
    if (up    and  left  and !valid_coord(y-1,x-1,garden,id)) region.sides += 1; 
    if (down  and  right and !valid_coord(y+1,x+1,garden,id)) region.sides += 1; 
    if (down  and  left  and !valid_coord(y+1,x-1,garden,id)) region.sides += 1; 

    region.area += 1;
}

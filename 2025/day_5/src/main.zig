const std = @import("std");

const input = @embedFile("test.txt");

const Range = struct {
    low: usize,
    high: usize,
};

fn ascending(v: void, a: Range, b: Range) bool {
    _ = v;
    return a.low < b.low;
}

fn binary_search(ranges: []const Range, ingredient: usize) ?usize {
    var low: usize = 0;
    var high: usize = ranges.len;

    if (high == 0 or ingredient < ranges[0].low or ingredient > ranges[high-1].high) return null;

    while (low < high) {
        const mid = (low + high) >> 1;
        const value = ranges[mid].low;
        if (ingredient > value) {
            low = mid + 1;
        } else if (ingredient == value) {
            return mid;
        } else {
            high = mid;
        }
    }

    return low - 1;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{.safety = false}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;
     
    var blocks = comptime std.mem.tokenizeSequence(u8, input, "\n\n");

    var range_list = std.mem.tokenizeAny(u8, blocks.next().?, "-\n");
    var ingredients = std.mem.tokenizeScalar(u8, blocks.rest(), '\n');

    var ranges = try std.ArrayList(Range).initCapacity(allocator, 10);
    defer ranges.deinit(allocator);

    while (range_list.next()) |low_str| {
        const low = try std.fmt.parseInt(usize, low_str, 10);
        const high = try std.fmt.parseInt(usize, range_list.next().?, 10);
        try ranges.append(allocator, .{.low=low,.high=high});
    }
    //ranges can overlap, so we sort then merge overlaps
    std.mem.sort(Range, ranges.items, {}, ascending);
    
    var merged_ranges = try std.ArrayList(Range).initCapacity(allocator, ranges.items.len);
    defer merged_ranges.deinit(allocator);
    try merged_ranges.append(allocator, ranges.items[0]);

    for (1..ranges.items.len) |i| {
        const len = merged_ranges.items.len;
        if (merged_ranges.items[len-1].high >= ranges.items[i].low) {
            merged_ranges.items[len-1].high = @max(merged_ranges.items[len-1].high, 
                                             ranges.items[i].high);        
        } else {
            try merged_ranges.append(allocator, ranges.items[i]);
        }
    }
    
    var part2: usize = 0;
    for (merged_ranges.items) |range| {
        part2 += range.high - range.low + 1;
    }

    var part1: usize = 0;
    while (ingredients.next()) |ingredient_str| {
        const ingredient = try std.fmt.parseInt(usize, ingredient_str, 10);

        if (binary_search(merged_ranges.items, ingredient)) |index| {
            if (ingredient <= merged_ranges.items[index].high) {
                part1 += 1;
            }
        }
    }

    try stdout.print("Part 1: {} - Part 2: {}\n", .{part1, part2});

    try stdout.flush();
}

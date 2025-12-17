const std = @import("std");

const input = @embedFile("test.txt");

const Position = struct {
    y: usize,
    x: usize,
};

const width = std.mem.indexOf(u8, input, "\n").? + 1;
const height = input.len / width;

var memo: [height][width-1]?usize = std.mem.zeroes([height][width-1]?usize);

fn calculate_timelines(y: usize, x: usize) usize {
    if (memo[y][x]) |cached| return cached;
    
    const char = input[x+y*width];
    
    const result: usize = switch(char) {
        '.', 'S' => blk: { 
            var total: usize = 0;
            var next_y = y;
            while (next_y < height - 1): (next_y+=1) {
                const next_char = input[x+next_y*width];
                if (next_char == '^') {
                    if (x > 0) total += calculate_timelines(next_y, x - 1);
                    if (x + 1 < width - 1) total += calculate_timelines(next_y, x + 1);
                    break :blk total;
                }
            }
            break :blk 1;
        },
        else => 0,
    };

    memo[y][x] = result;
    return result;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{.safety = false}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    const start = std.mem.indexOf(u8, input, "S").?;

    var visited: [height][width-1]bool = std.mem.zeroes([height][width-1]bool);

    //manage unvisited nodes with stack dfs
    var stack = try std.ArrayList(Position).initCapacity(allocator, 16);
    defer stack.deinit(allocator);
    try stack.append(allocator, .{.y=0, .x=start});
    
    var part1: usize = 0;
    while (stack.pop()) |beam| {
        visited[beam.y][beam.x] = true;
        var y = beam.y+1;
        while (y < height) : (y += 1) {
            if (input[beam.x+y*width] == '^') {
                var split: bool = false;
                //spawn two new beams on either side of beam splitter
                if (beam.x > 0 and visited[y][beam.x-1] == false) {
                    try stack.append(allocator, .{.y=y,.x=beam.x-1});
                    visited[y][beam.x-1] = true; 
                    split = true;
                }
                if (beam.x+1 < width-1 and visited[y][beam.x+1] == false) {
                    try stack.append(allocator, .{.y=y, .x=beam.x+1});
                    visited[y][beam.x-1] = true; 
                    split = true;
                }
                if (split) part1 += 1;
                break;
            }
        }

    }

    const part2 = calculate_timelines(0, start);

    try stdout.print("Part 1: {} - Part 2: {}\n", .{part1, part2});
    try stdout.flush();
}

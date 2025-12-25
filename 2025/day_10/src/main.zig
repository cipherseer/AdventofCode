const std = @import("std");

const input = @embedFile("trial.txt");

pub fn main() !void {
    var arena = std.heap.ArenaAllocator.init(std.heap.page_allocator);
    defer arena.deinit();
    const allocator = arena.allocator();
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;

    var lines = std.mem.tokenizeScalar(u8, input, '\n');

    while (lines.next()) |line| {
        defer _ = arena.reset(.retain_capacity);
        var sections = std.mem.tokenizeAny(u8, line[1..line.len-1], "]{");
        const diagram_str = sections.next().?;
        const raw_button_str = sections.next().?;
        var button_strs = std.mem.tokenizeSequence(u8, raw_button_str[2..raw_button_str.len-2], ") (");
        const raw_joltage_str = sections.next().?;
        var joltage_strs = std.mem.tokenizeScalar(u8, raw_joltage_str, ',');

        //parse indicator light target
        var light_target: []bool = try allocator.alloc(bool, diagram_str.len);
        @memset(light_target, false);
        for (diagram_str, 0..) |c, i| {
            switch (c) {
                '#' => light_target[i] = true,
                else => {}
            }
        }
        try stdout.print("{any}\n", .{light_target});
        //parse list of joltages
        var joltages: []isize = try allocator.alloc(isize, diagram_str.len);
        
        var i: usize = 0;
        while (joltage_strs.next()) |joltage_str| : (i+=1) {
            const joltage = try std.fmt.parseInt(isize, joltage_str, 10);
            joltages[i] = joltage;
        }
        try stdout.print("{any}\n", .{joltages});
        
        //parse wiring of light switches
        var buttons = try std.ArrayList(usize).initCapacity(allocator, 8);
        while (button_strs.next()) |button_str| {
            var light_numbers = std.mem.tokenizeScalar(u8, button_str, ',');
            var button: usize = 0;
            while (light_numbers.next()) |light_number| {
                button += @as(usize, @intCast(1)) << @intCast(light_number[0]-48);
            }
            try buttons.append(allocator, button);
            try stdout.print("{b} ", .{button});
        }
        try stdout.print("\n", .{});
        
    }
    try stdout.flush();
}


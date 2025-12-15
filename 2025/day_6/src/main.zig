const std = @import("std");

const input = @embedFile("test.txt");

const Operator = enum {
    Add,
    Mul,

    fn identity(self: Operator) usize {
        return switch(self) {
            .Add => 0,
            .Mul => 1,
        };
    }

    fn apply(self: Operator, a: usize, b: usize) usize {
        return switch(self) {
            .Add => a + b,
            .Mul => a * b,
        };
    }
};

fn parse_operator(char: u8) ?Operator {
    return switch(char) {
        '+' => .Add,
        '*' => .Mul,
        else => null,
    };
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{.safety = false}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var stdout_buffer: [1024]u8 = undefined;
    var stdout_writer = std.fs.File.stdout().writer(&stdout_buffer);
    const stdout = &stdout_writer.interface;
    
    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    const width = std.mem.indexOf(u8, input, "\n").? + 1;
    const num_lines: usize = input.len / width - 1;

    var numbers = try std.ArrayList(usize).initCapacity(allocator, 64);
    defer numbers.deinit(allocator);

    outer: while (lines.peek()) |line|{
        var number_list = std.mem.tokenizeScalar(u8, line, ' ');
        while (number_list.next()) |number_str| {
            const number = std.fmt.parseInt(usize, number_str, 10) catch break :outer;
            try numbers.append(allocator, number);
        }
        _ = lines.next();
    }

    const num_equations = numbers.items.len / num_lines;
    var operations = std.mem.tokenizeScalar(u8, lines.rest(), ' ');
    
    var equation: usize = 0;
    var part1: usize = 0;
    var part2: usize = 0;
    while (operations.next()) |operation| : (equation+=1) {
        //operation is positioned in the first column
        var column: usize = operations.index - 1;
        
        const op: Operator = parse_operator(operation[0]) orelse break;
        //part 1
        var total: usize = op.identity();
        for (0..num_lines) |i| {
           total = op.apply(total, numbers.items[equation+i*num_equations]);
        }
        part1 += total;

        //part 2
        var total2: usize = op.identity();

        while (true): (column+=1) {
            var number: usize = 0;
            var empty_count: usize = 0;
            for (0..num_lines) |row| {
                const digit = input[column+row*width];
                if (digit < '0' or digit > '9') {
                    empty_count += 1;
                } else {
                    number = 10*number + digit - '0';
                }
            }
            if (empty_count == num_lines) break;
            total2 = op.apply(total2, number);
        }
        part2 += total2;
    }
    try stdout.print("Part 1 - {} Part 2 - {}\n", .{part1, part2});
    try stdout.flush();
}

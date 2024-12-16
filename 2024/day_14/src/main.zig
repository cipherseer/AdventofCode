const std = @import("std");
const rl = @import("raylib");

const input = @embedFile("input.txt");

const Robot = struct {
    y: i32,
    x: i32,
    vy: i32,
    vx: i32,

    pub fn init(y: i32, x: i32, vy: i32, vx: i32) Robot {
        return .{.y = y, .x = x, .vy = vy, .vx = vx};
    }

    pub fn simulate(self: *Robot, steps: i32) void {
        self.x = @mod(self.x + steps*self.vx, cols);
        self.y = @mod(self.y + steps*self.vy, rows);
    }
};



const cols: u32 = 101;
const rows: u32 = 103;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();

    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();
    
    var lines = std.mem.tokenizeScalar(u8, input, '\n');
    


    var count: @Vector(4, u32) = @splat(0);

    var robots = std.ArrayList(Robot).init(allocator);
    defer robots.deinit();

    while (lines.next()) |line| {
        var numbers = std.mem.tokenizeAny(u8, line, "p=, v");
        var x: i32 = try std.fmt.parseInt(i32, numbers.next().?, 10);
        var y: i32 = try std.fmt.parseInt(i32, numbers.next().?, 10);
        const vx: i32 = try std.fmt.parseInt(i32, numbers.next().?, 10);
        const vy: i32 = try std.fmt.parseInt(i32, numbers.next().?, 10);
        
        try robots.append(Robot.init(y, x, vy, vx));

        x = @mod(x + 100*vx, cols);
        y = @mod(y + 100*vy, rows);
        
        if (x == cols / 2 or y == rows / 2) continue;

        const x_quad: u2 = @as(u2, @intFromBool(x > cols / 2)) << 1;
        const y_quad: u2 = @as(u2, @intFromBool(y > rows / 2));
        const quadrant = x_quad | y_quad;

        switch (quadrant) {
            0 => count[0] += 1,
            1 => count[1] += 1,
            2 => count[2] += 1,
            3 => count[3] += 1,
        }
    }

    const screen_width = cols*10;
    const screen_height = rows*10 + 30;
    
    var steps_buffer: [100]u8 = undefined;

    rl.initWindow(screen_width, screen_height, "Robot Simulation");
    defer rl.closeWindow();

    var grid: [rows][cols]bool = std.mem.zeroes([rows][cols]bool);
    var step: usize = 0;
    var paused: bool = false;

    while (!rl.windowShouldClose()) {

        if (!paused) {
            step += 1;
            //reset grid
            grid = std.mem.zeroes([rows][cols]bool);

            for (robots.items) |*robot| {
                robot.simulate(1);
                grid[@intCast(robot.y)][@intCast(robot.x)] = true;
            }

            outer: for (grid, 0..) |row, i| {
                var robo_count: usize = 0;
                for (row, 0..) |robot, j| {
                    if (robot) {
                        robo_count += 1;
                    } else {
                        robo_count = 0;
                    }

                    if (robo_count >= 20) {
                        std.debug.print("row detected on step {} row: {} col: {}\n", .{step, i, j});
                        paused = true;
                        break :outer;
                    }
                }
            }
        }

        if (rl.isKeyPressed(.key_space)) {
            paused = !paused;
        }
        
        if (paused) {
            if (rl.isKeyPressed(.key_right)) {
                step += 1;
                for (robots.items) |*robot| robot.simulate(1);
            } else if (rl.isKeyPressed(.key_one)) {
                step += 100;
                for (robots.items) |*robot| robot.simulate(100);
            } else if (rl.isKeyPressed(.key_left)) {
                if (step > 0) {
                    step -= 1;
                    for (robots.items) |*robot| robot.simulate(-1);
                }
            } else if (rl.isKeyPressed(.key_two)) {
                if (step > 100) {
                    step -= 100;
                    for (robots.items) |*robot| robot.simulate(-100);
                }
            } 
        }

        rl.beginDrawing();
        defer rl.endDrawing();

        rl.clearBackground(rl.Color.black);
        const slice = try std.fmt.bufPrintZ(&steps_buffer, "right: 1, left: -1, 1: 100, 2: -100, Step: {}", .{step});
        rl.drawText(slice, 10, 5, 15, rl.Color.white);
        
        for (robots.items) |robot| {
            const rect_x = robot.x * 10;
            const rect_y = robot.y * 10 + 30;
            rl.drawRectangle(rect_x, rect_y, 10, 10, rl.Color.green);
        }
    }


    try stdout.print("Part 1: {}\n", .{@reduce(std.builtin.ReduceOp.Mul, count)});
    try bw.flush();
}

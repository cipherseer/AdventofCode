const std = @import("std");

const input = @embedFile("input.txt");

pub const RuleBook = struct {
    allocator: std.mem.Allocator,
    map: std.AutoHashMap(u8, std.AutoHashMap(u8, void)),

    pub fn init(allocator: std.mem.Allocator) RuleBook {
        const map = std.AutoHashMap(u8, std.AutoHashMap(u8, void)).init(allocator);
        return RuleBook{ .allocator = allocator, .map = map };
    }

    pub fn add(self: *RuleBook, key: u8, value: u8) !void {
        var entry = try self.map.getOrPut(key);

        if (!entry.found_existing) {
            entry.value_ptr.* = std.AutoHashMap(u8, void).init(self.allocator);
        }

        try entry.value_ptr.put(value, {});
    }

    pub fn contains(self: *RuleBook, key: u8, value: u8) bool {
        const entry_option = self.map.get(key);
        if (entry_option) |entry| {
            return entry.contains(value);
        } else {
            return false;
        }
    }

    pub fn print(self: *RuleBook) void {
        var entries = self.map.iterator();
        while (entries.next()) |entry| {
            std.debug.print("key: {}\n", .{entry.key_ptr.*});
            var values = entry.value_ptr.iterator();

            while (values.next()) |value| {
                std.debug.print("{} ", .{value.key_ptr.*});
            }
            std.debug.print("\n", .{});
        }
    }

    pub fn deinit(self: *RuleBook) void {
        var it = self.map.iterator();
        while (it.next()) |entry| {
            entry.value_ptr.deinit();
        }

        self.map.deinit();
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

    var rulebook: RuleBook = RuleBook.init(allocator);
    defer rulebook.deinit();

    var sections = std.mem.splitSequence(u8, input, "\n\n");
    var rules = std.mem.splitScalar(u8, sections.first(), '\n');
    var updates = std.mem.tokenizeScalar(u8, sections.rest(), '\n');
    while (rules.next()) |rule| {
        var nums = std.mem.splitScalar(u8, rule, '|');
        const key = try std.fmt.parseInt(u8, nums.first(), 10);
        const value = try std.fmt.parseInt(u8, nums.rest(), 10);
        try rulebook.add(key, value);
    }

    // rulebook.print();

    var buffer: [80]u8 = undefined;
    var fba = std.heap.FixedBufferAllocator.init(&buffer);
    const fba_allocator = fba.allocator();
    var result1: u32 = 0;
    var result2: u32 = 0;
    while (updates.next()) |update| {
        var previous_pages = std.ArrayList(u8).init(fba_allocator);
        defer previous_pages.deinit();
        var pages = std.mem.tokenizeScalar(u8, update, ',');
        var valid_update: bool = true;

        while (pages.next()) |slice| {
            const page = try std.fmt.parseInt(u8, slice, 10);
            for (previous_pages.items) |previous| {
                if (rulebook.contains(page, previous)) {
                    valid_update = false;
                }
            }
            try previous_pages.append(page);
        }

        if (valid_update) {
            result1 += @intCast(previous_pages.items[previous_pages.items.len / 2]);
        } else {
            std.mem.sort(u8, previous_pages.items, &rulebook, compare_pages);
            // try stdout.print("{any}\n", .{previous_pages.items});
            result2 += @intCast(previous_pages.items[previous_pages.items.len / 2]);
        }
    }

    try stdout.print("Part 1: {}\n", .{result1});
    try stdout.print("Part 2: {}\n", .{result2});
    try bw.flush();
}

fn compare_pages(rules: *RuleBook, a: u8, b: u8) bool {
    return rules.contains(a, b);
}

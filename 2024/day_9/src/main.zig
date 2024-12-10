const std = @import("std");

const input = @embedFile("input.txt");

const Free = struct {
    index: u32,
    size: u8,
};

const File = struct {
    index: u32,
    id: u16,
    size: u8,

    pub fn checksum(self: File) u32 {
        return self.id * (self.index * self.size + self.size * (self.size - 1) / 2);
    }
};

const FreeList = std.ArrayListUnmanaged(Free);
const FileList = std.ArrayListUnmanaged(File);

fn parse(allocator: std.mem.Allocator, buffer: []const u8) !struct { FileList, FreeList } {
    var files = try FileList.initCapacity(allocator, buffer.len / 2 + 1);
    var frees = try FreeList.initCapacity(allocator, buffer.len / 2);
    var index: u32 = 0;

    for (0..buffer.len, buffer) |i, c| {
        const n: u8 = c - '0';
        if (i % 2 == 0) {
            files.appendAssumeCapacity(.{ .index = index, .id = @intCast(i / 2), .size = n });
        } else if (n != 0) {
            frees.appendAssumeCapacity(.{ .index = index, .size = n });
        }
        index += n;
    }

    return .{ files, frees };
}

fn part1(files: []File, frees: []Free) usize {
    var result: usize = 0;
    var right: usize = files.len - 1;
    var free: usize = 0;

    while (frees[free].index < files[right].index + files[right].size - 1) {
        const realloced = @min(files[right].size, frees[free].size);

        result += File.checksum(.{ .index = frees[free].index, .id = files[right].id, .size = realloced });

        files[right].size -= realloced;
        if (files[right].size == 0) right -= 1;

        frees[free].size -= realloced;
        frees[free].index += realloced;
        if (frees[free].size == 0) free += 1;
    }

    //check if final file was fragmented
    if (files[right].size > 0) result += files[right].checksum();

    //calculate checksum for remaining files
    for (0..right) |i| {
        result += files[i].checksum();
    }

    return result;
}

fn part2(files: []File, frees: *FreeList) usize {
    var result: usize = 0;
    var right = files.len - 1;

    while (right > 0) {
        for (0..frees.items.len, frees.items) |i, free| {
            if (free.index >= files[right].index) break;
            if (free.size >= files[right].size) {
                files[right].index = free.index;
                frees.items[i].size -= files[right].size;
                if (frees.items[i].size == 0) {
                    _ = frees.orderedRemove(i);
                } else frees.items[i].index += files[right].size;
                break;
            }
        }
        right -= 1;
    }

    for (files) |file| {
        result += file.checksum();
    }

    return result;
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();
    defer {
        _ = gpa.deinit();
    }

    const stdout_file = std.io.getStdOut().writer();
    var bw = std.io.bufferedWriter(stdout_file);
    const stdout = bw.writer();

    var files, var frees = try parse(allocator, input[0 .. input.len - 1]);
    defer files.deinit(allocator);
    defer frees.deinit(allocator);

    //copy files and frees for Part 2
    var files2 = try files.clone(allocator);
    defer files2.deinit(allocator);
    var frees2 = try frees.clone(allocator);
    defer frees2.deinit(allocator);

    //Part 1

    //Part 2

    try stdout.print("Part 1: {}\n", .{part1(files.items, frees.items)});
    try stdout.print("Part 2: {}\n", .{part2(files2.items, &frees2)});
    try bw.flush();
}

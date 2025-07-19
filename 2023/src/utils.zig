const std = @import("std");
const Allocator = std.mem.Allocator;

// Lots hre copied from: https://github.com/p88h/aoc2024/tree/main

pub fn read_file(allocator: Allocator, filename: []const u8) []u8 {
    // potentially common stuff
    std.log.debug("{}", .{std.fs.cwd().fd});
    var file = std.fs.cwd().openFile(filename, .{}) catch {
        std.debug.panic("file not found: {s}\n", .{filename});
    };
    defer file.close();
    return file.readToEndAlloc(allocator, 232072) catch {
        std.debug.panic("Error reading: {s}\n", .{filename});
    };
}


pub fn split_lines(allocator: Allocator, buf: []u8) [][]const u8 {
    var lines = std.ArrayList([]const u8).init(allocator);
    var iter = std.mem.splitAny(u8, buf, "\n");
    while (iter.next()) |line| lines.append(line) catch unreachable;
    // remove last line if empty
    if (lines.items[lines.items.len - 1].len == 0) _ = lines.pop();
    return lines.items;
}
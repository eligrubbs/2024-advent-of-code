const std = @import("std");
const Allocator = std.mem.Allocator;

const utils = @import("../utils.zig");


fn parse_line(line: []const u8) u8 {
    var left_int: u8 = 10;
    var right_int: u8 = 10;

    fwd_walk: for (line) |x| {
        if (x >= '0' and x <= '9') {
            left_int = std.fmt.charToDigit(x, 10) catch unreachable;
            break :fwd_walk;
        }
    }
    if (left_int >= 10) {
        std.log.err("{s}", .{line});
        unreachable;
    }

    rev_walk: for (1..(line.len+1)) |i| {
        const x: u8 = line[line.len-i];
        if (x >= '0' and x <= '9') {
            right_int = std.fmt.charToDigit( x, 10) catch unreachable;
            break :rev_walk;
        }
    }
    if (right_int >= 10) {
        std.log.err("{s}", .{line});
        unreachable;
    }
    
    return (left_int * 10) + right_int;
}

pub fn day_1_p1(allocator: Allocator) void {

    const file: []u8 = utils.read_file(allocator, "./src/data/d1_p1.txt");

    const lines: [][]const u8 = utils.split_lines(allocator, file);

    var total: u128 = 0;

    for (lines) |line| {
        total += parse_line(line);
    }
    
    std.log.debug("Day 1 P1: {d}", .{total});
}

pub fn day_1_p2(allocator: Allocator) void {
    const file: []u8 = utils.read_file(allocator, "./src/data/d1_p1.txt");

    const lines: [][]const u8 = utils.split_lines(allocator, file);
    _ = lines;
    var total: u128 = 0;

    std.log.debug("Day 1 P2: {d}", .{total});
}


const expect = std.testing.expect;
test "test_p1_parse_line" {
    try expect( parse_line("20") == 20);
    try expect( parse_line("1abc2") == 12);
    try expect( parse_line("pqr3stu8vwx") == 38);
    try expect( parse_line("a1b2c3d4e5f") == 15);
    try expect( parse_line("treb7uchet") == 77);
}

//! By convention, root.zig is the root source file when making a library. If
//! you are making an executable, the convention is to delete this file and
//! start with main.zig instead.
const std = @import("std");
const Allocator = std.mem.Allocator;

pub const day_1 = @import("./days/day_1.zig");

pub const Day = enum(u8) {
    D01,
    D02,
    D03,
    D04,
    D05,
    D06,
    D07,
    D08,
    D09,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
    D16,
    D17,
    D18,
    D19,
    D20,
    D21,
    D22,
    D23,
    D24,
    D25,

    pub const DayError = error{InvalidDay};

    pub fn from_string(str: [:0]const u8) !Day {
        const val: Day = std.meta.stringToEnum(Day, str) orelse {
            return DayError.InvalidDay;
        };
        return val;
    }
};


pub fn run_specific_day(allocator: Allocator, day: Day) void {
    switch (day) {
        Day.D01 => {
            day_1.day_1_p1(allocator);
            day_1.day_1_p2();
        },
        Day.D02 => {},
        Day.D03 => {},
        Day.D04 => {},
        Day.D05 => {},
        Day.D06 => {},
        Day.D07 => {},
        Day.D08 => {},
        Day.D09 => {},
        Day.D10 => {},
        Day.D11 => {},
        Day.D12 => {},
        Day.D13 => {},
        Day.D14 => {},
        Day.D15 => {},
        Day.D16 => {},
        Day.D17 => {},
        Day.D18 => {},
        Day.D19 => {},
        Day.D20 => {},
        Day.D21 => {},
        Day.D22 => {},
        Day.D23 => {},
        Day.D24 => {},
        Day.D25 => {},
    }
}
//! By convention, main.zig is where your main function lives in the case that
//! you are building an executable. If you are making a library, the convention
//! is to delete this file and start with root.zig instead.

const std = @import("std");
/// This imports the separate module containing `root.zig`. Take a look in `build.zig` for details.
const lib = @import("advent_days");

pub fn main() !void {
    // Initialize Allocator
    const allocator = std.heap.page_allocator;

    // init args
    var args = try std.process.ArgIterator.initWithAllocator(allocator);
    defer args.deinit();
    _ = args.next();

    var stop_running: bool = false;

    arg_loop: while (args.next()) |x| {
        if (std.mem.eql(u8, x, "all")) {
            stop_running = true;
        } else {
            const entry = lib.Day.from_string(x) catch {
                std.log.debug("Invalid day, skipping: {s}!", .{x});
                continue :arg_loop;
            };
            std.log.debug("{:}!", .{entry});
            lib.run_specific_day(allocator, entry);
        }
        
        if (stop_running) {
            break :arg_loop;
        }
    }

}


/// Ensure that the entered day string literal falls between `01` and `25` inclusive.
fn is_valid_entered_day(x: [:0]const u8) bool {
    return (
        (x.len == 2) and
        (
            (x[0] >= '0' and x[0] <= '2' and x[1] >= '0' and x[1] <= '9') and
            (
                (x[0] == '2' and x[1] <= '5') or
                !(x[0] == '0' and x[1] == '0')
            )
        )
    );
}

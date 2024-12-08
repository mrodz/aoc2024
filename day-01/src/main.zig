const std = @import("std");
const root = @import("root.zig");

const DEBUG = false; 

pub fn partOne(input: *const root.LineInput, allocator: std.mem.Allocator) !void {
    const first, const second = input.intoLocations(allocator) catch |err| {
        std.log.err("Could not split into locations", .{});
        return err;
    };
    defer {
        allocator.free(first);
        allocator.free(second);
    }

    root.quicksort(first);
    root.quicksort(second);

    var sum: u32 = 0;

    for (0..input.len()) |i| {
        sum += @abs(@max(second[i], first[i]) - @min(second[i], first[i]));
    }

    std.debug.print("What is the total distance between your lists?\n\nThe Answer is: {any}\n", .{sum});
}

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer if (gpa.deinit() == .leak) @panic("Memory Leak");

    var input = root.parseLinesFromFile("./input.txt", gpa.allocator()) catch |err| {
        std.log.err("Could not parse data in `input.txt`", .{});
        return err;
    };
    defer input.deinit();

    try partOne(&input, gpa.allocator());
}

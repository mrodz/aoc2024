const std = @import("std");
const root = @import("root.zig");

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
        sum += @max(second[i], first[i]) - @min(second[i], first[i]);
    }

    std.debug.print("What is the total distance between your lists?\nThe Answer is: {any}\n", .{sum});
}

pub fn partTwo(input: *const root.LineInput, allocator: std.mem.Allocator) !void {
    const first, const second = input.intoLocations(allocator) catch |err| {
        std.log.err("Could not split into locations", .{});
        return err;
    };
    defer {
        allocator.free(first);
        allocator.free(second);
    }

    var map = std.AutoHashMap(u32, u16).init(allocator);
    defer map.deinit();

    for (second) |location| {
        if (map.getEntry(location)) |entry| entry.value_ptr.* += 1 else try map.put(location, 1);
    }

    var similarityScore: u32 = 0;

    for (first) |location| {
        if (map.get(location)) |repetitions| {
            similarityScore += location * repetitions;
        }
    }

    std.debug.print("Once again consider your left and right lists. What is their similarity score?\nThe Answer is: {any}\n", .{similarityScore});
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
    try partTwo(&input, gpa.allocator());
}

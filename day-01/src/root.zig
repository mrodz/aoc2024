const std = @import("std");

pub const LineIterator = struct {
    index: usize,
    input: *LineInput,

    pub fn init(input: *LineInput) LineIterator {
        return .{
            .input = input,
            .index = 0,
        };
    }

    pub fn next(self: *LineIterator) ?[2]u32 {
        const element = self.input.get(self.index) orelse return null;
        self.index += 1;
        return element;
    }
};

pub const LineInput = struct {
    lines: std.ArrayList([2]u32),

    pub fn init(allocator: std.mem.Allocator) LineInput {
        return .{ .lines = std.ArrayList([2]u32).init(allocator) };
    }

    pub fn appendTuple(self: *LineInput, tuple: [2]u32) !void {
        try self.lines.append(tuple);
    }

    pub fn deinit(self: *LineInput) void {
        self.lines.deinit();
        self.* = undefined;
    }

    pub fn get(self: *const LineInput, index: usize) ?[2]u32 {
        if (index < self.len()) return self.lines.items[index];
        return null;
    }

    pub fn len(self: *const LineInput) usize {
        return self.lines.items.len;
    }

    pub fn iter(self: *const LineInput) LineIterator {
        return .{
            .input = self,
            .index = 0,
        };
    }

    pub fn intoLocations(self: *const LineInput, allocator: std.mem.Allocator) ![2][]u32 {
        const left = try allocator.alloc(u32, self.len());
        errdefer allocator.free(left);
        const right = try allocator.alloc(u32, self.len());
        errdefer allocator.free(right);

        for (self.lines.items, 0..) |tuple, i| {
            left[i] = tuple[0];
            right[i] = tuple[1];
        }

        return .{ left, right };
    }
};

const InputParseError = error{
    LineMissingComponents,
};

pub fn parseLinesFromFile(file_path: []const u8, allocator: std.mem.Allocator) !LineInput {
    var lineInput = LineInput.init(allocator);
    errdefer lineInput.deinit();

    var file = try std.fs.cwd().openFile(file_path, .{});
    defer file.close();

    var bufferedReader = std.io.bufferedReader(file.reader());
    var stream = bufferedReader.reader();

    var buffer: [128]u8 = undefined;

    while (try stream.readUntilDelimiterOrEof(&buffer, '\n')) |raw_line| : (buffer = undefined) {
        errdefer std.log.err("`{s}`", .{raw_line});

        const line = if (raw_line[raw_line.len - 1] == '\r') raw_line[0 .. raw_line.len - 1] else raw_line;

        var iter = std.mem.splitScalar(u8, line, 32);
        const first_num_s = iter.next() orelse return InputParseError.LineMissingComponents;

        const second_num_s = while (iter.next()) |maybe_next| {
            if (maybe_next.len == 0) continue;
            break maybe_next;
        } else {
            return InputParseError.LineMissingComponents;
        };

        errdefer std.log.err("`{s}` and `{s}`", .{ first_num_s, second_num_s });
        const first_num = try std.fmt.parseInt(u32, first_num_s, 10);
        const second_num = try std.fmt.parseInt(u32, second_num_s, 10);

        try lineInput.appendTuple(.{ first_num, second_num });
    }

    return lineInput;
}

pub fn quicksort(array: []u32) void {
    sort(array, 0, array.len - 1);
}

fn sort(array: []u32, lo: usize, hi: usize) void {
    if (lo < hi) {
        const p = partition(array, lo, hi);
        sort(array, lo, @min(p, p -% 1));
        sort(array, p + 1, hi);
    }
}

fn partition(A: []u32, lo: usize, hi: usize) usize {
    //Pivot can be chosen otherwise, for example try picking the first or random
    //and check in which way that affects the performance of the sorting
    const pivot = A[hi];
    var i = lo;
    var j = lo;
    while (j < hi) : (j += 1) {
        if (A[j] < pivot) {
            std.mem.swap(u32, &A[i], &A[j]);
            i = i + 1;
        }
    }
    std.mem.swap(u32, &A[i], &A[hi]);
    return i;
}

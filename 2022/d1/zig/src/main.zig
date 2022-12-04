const std = @import("std");

const testFile = @embedFile("../../test");
const inputFile = @embedFile("../../input");

fn parse(raw: []const u8, allocator: std.mem.Allocator) !std.ArrayList(u32) {
    var elves = std.ArrayList(u32).init(allocator);

    var lines = std.mem.split(u8, raw, "\n");
    var currentElf: u32 = 0;

    while (lines.next()) |line| {
        if (line.len == 0) {
            try elves.append(currentElf);
            currentElf = 0;
        } else {
            var parsed = try std.fmt.parseUnsigned(u32, line, 10);
            currentElf = currentElf + parsed;
        }
    }

    if (currentElf > 0) {
        try elves.append(currentElf);
    }

    return elves;
}

fn run(label: []const u8, data: []const u8, allocator: std.mem.Allocator) !void {
    std.log.info("RUNNING: {s}\n", .{label});

    var parsed = blk: {
        var x = try parse(data, allocator);
        break :blk x.toOwnedSlice();
    };

    std.sort.sort(u32, parsed, {}, comptime std.sort.desc(u32));

    std.log.info("max calories held: {any}", .{parsed[0]});
    std.log.info("held by top 3: {any}", .{parsed[0] + parsed[1] + parsed[2]});
}

pub fn main() anyerror!void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    try run("test", testFile, allocator);
    try run("actual", inputFile, allocator);
}

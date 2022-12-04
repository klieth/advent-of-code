const std = @import("std");

const inputFile = @embedFile("../../input");

const Turn = enum {
    left,
    right,
};

const Command = struct {
    turn: Turn,
    distance: u32,
};

const Location = struct {
    x: i32,
    y: i32,

    fn distance(self: Location) !i32 {
        var x = try std.math.absInt(self.x);
        var y = try std.math.absInt(self.y);
        return x + y;
    }
};

const Direction = enum(u2) {
    north,
    east,
    south,
    west,

    fn turn(self: Direction, to: Turn) Direction {
        var val = @enumToInt(self);
        var res = switch (to) {
            Turn.left => val -% 1,
            Turn.right => val +% 1,
        };
        return @intToEnum(Direction, res);
    }
};

const Position = struct {
    loc: Location,
    dir: Direction,

    fn origin() Position {
        var loc = Location{
            .x = 0,
            .y = 0
        };

        return Position{
            .loc = loc,
            .dir = Direction.north,
        };
    }

    fn turn(self: *Position, t: Turn) void {
        self.dir = self.dir.turn(t);
    }

    fn step(self: *Position, len: i32) void {
        switch (self.dir) {
            Direction.north => self.loc.y += len,
            Direction.east  => self.loc.x += len,
            Direction.south => self.loc.y -= len,
            Direction.west  => self.loc.x -= len,
        }
    }

    fn apply(self: *Position, command: Command) void {
        self.turn(command.turn);
        self.step(@intCast(i32, command.distance));
    }
};

fn parse(allocator: std.mem.Allocator, raw: []const u8) !std.ArrayList(Command) {
    var commands = std.ArrayList(Command).init(allocator);

    var raw_commands = std.mem.split(u8, std.mem.trim(u8, raw, "\n"), ", ");

    while (raw_commands.next()) |rd| {
        var turn = switch (rd[0]) {
            'R' => Turn.right,
            'L' => Turn.left,
            else => return error.BadCommand
        };

        var distance = try std.fmt.parseUnsigned(u32, rd[1..], 10);

        try commands.append(.{ .turn = turn, .distance = distance });
    }

    return commands;
}

fn part1(commands: []const Command) Position {
    var position = Position.origin();

    for (commands) |command| {
        position.apply(command);
    }

    return position;
}

fn part2(allocator: std.mem.Allocator, commands: []const Command) !Position {
    var visited = std.AutoHashMap(Location, void).init(allocator);

    var position = Position.origin();
    try visited.put(position.loc, {});

    for (commands) |command| {
        position.turn(command.turn);

        var stepped: u32 = 0;
        while (stepped < command.distance): (stepped += 1) {
            position.step(1);

            if (visited.contains(position.loc)) {
                return position;
            }

            try visited.put(position.loc, {});
        }
    }

    return error.NotFound;
}

pub fn main() anyerror!void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    const allocator = gpa.allocator();

    const commands = blk: {
        var x = try parse(allocator, inputFile);
        break :blk x.toOwnedSlice();
    };

    //const testCommands = blk: {
    //    var x = try parse(allocator, "R8, R4, R4, R8");
    //    break :blk x.toOwnedSlice();
    //};

    std.log.info("part 1: {any}", .{part1(commands).loc.distance()});

    var p2_result = try part2(allocator, commands);
    std.log.info("part 2: {any}", .{p2_result.loc.distance()});
}

const std = @import("std");

pub fn main() !void {
    const input = @embedFile("day_21.txt");

    var debug_allocator: std.heap.DebugAllocator(.{}) = .init;
    const gpa = debug_allocator.allocator();

    std.debug.print("{d}\n", .{try solve(gpa, input)});
}

const Op = enum { add, sub, mul, div };
const Monkey = union(enum) { number: i64, expr: struct { op: Op, a: []const u8, b: []const u8 } };

fn solve(gpa: std.mem.Allocator, input: []const u8) !i64 {
    var lines = std.mem.tokenizeScalar(u8, input, '\n');

    var monkeys: std.StringHashMapUnmanaged(Monkey) = .empty;
    defer monkeys.deinit(gpa);

    while (lines.next()) |line| {
        var parts = std.mem.tokenizeAny(u8, line, ": ");

        const lhs = parts.next().?;
        const rhs = parts.next().?;

        if (parts.next()) |more| {
            const op: Op = switch (more[0]) {
                '+' => .add,
                '-' => .sub,
                '*' => .mul,
                '/' => .div,
                else => unreachable,
            };

            const b = parts.next().?;

            try monkeys.put(gpa, lhs, Monkey{ .expr = .{
                .a = rhs,
                .op = op,
                .b = b,
            } });
        } else {
            try monkeys.put(gpa, lhs, Monkey{ .number = try std.fmt.parseInt(i64, rhs, 10) });
        }
    }

    const root = monkeys.get("root").?;

    const known, const unknown = x: {
        const left = what_number(monkeys, root.expr.a);

        if (left) |l| {
            break :x .{ l, root.expr.b };
        }

        const right = what_number(monkeys, root.expr.b);

        break :x .{ right.?, root.expr.a };
    };

    return find_humn(monkeys, known, unknown);
}

fn find_humn(monkeys: std.StringHashMapUnmanaged(Monkey), target: i64, monkey: []const u8) i64 {
    if (std.mem.eql(u8, monkey, "humn")) {
        return target;
    }

    switch (monkeys.get(monkey).?) {
        .expr => |expr| {
            const a = what_number(monkeys, expr.a);
            const b = what_number(monkeys, expr.b);
            const new_target = switch (expr.op) {
                .add =>
                // a + x = target
                // x + b = target
                target - (a orelse b).?,
                .sub =>
                // a - x = target
                // x - b = target
                if (a) |_| -(target - a.?) else target + b.?,
                .mul =>
                // x * b = target
                // a * x = target
                @divExact(target, (a orelse b).?),
                .div =>
                // a / x = target
                // x / b = target
                if (a) |_| @divExact(a.?, target) else target * b.?,
            };

            return find_humn(monkeys, new_target, if (a == null) expr.a else expr.b);
        },
        else => unreachable,
    }
}

fn what_number(monkeys: std.StringHashMapUnmanaged(Monkey), monkey: []const u8) ?i64 {
    if (std.mem.eql(u8, monkey, "humn")) {
        return null;
    }
    switch (monkeys.get(monkey).?) {
        .number => |n| {
            return n;
        },
        .expr => |expr| {
            const a_or_null = what_number(monkeys, expr.a);

            const b_or_null = what_number(monkeys, expr.b);

            if (a_or_null) |a| {
                if (b_or_null) |b| {
                    return switch (expr.op) {
                        .add => a + b,
                        .sub => a - b,
                        .mul => a * b,
                        .div => @divExact(a, b),
                    };
                }
            }
            return null;
        },
    }
}

test {
    try std.testing.expectEqual(301, try solve(std.testing.allocator,
        \\root: pppw + sjmn
        \\dbpl: 5
        \\cczh: sllz + lgvd
        \\zczc: 2
        \\ptdq: humn - dvpt
        \\dvpt: 3
        \\lfqf: 4
        \\humn: 5
        \\ljgn: 2
        \\sjmn: drzm * dbpl
        \\sllz: 4
        \\pppw: cczh / lfqf
        \\lgvd: ljgn * ptdq
        \\drzm: hmdt - zczc
        \\hmdt: 32
        \\
    ));
}

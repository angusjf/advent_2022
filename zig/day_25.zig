const std = @import("std");

pub fn main() !void {
    const input = @embedFile("day_25.txt");
    var buf: [100]u8 = undefined;

    std.debug.print("{s}\n", .{solve(input, &buf)});
}

fn solve(input: []const u8, buf: []u8) []u8 {
    var lines = std.mem.tokenizeScalar(u8, input, '\n');

    var total: i64 = 0;

    while (lines.next()) |line| {
        total += snafuToDecimal(line);
    }

    std.debug.print("[in decimal {d}]\n", .{total});

    const result = decimalToSnafu(total, buf);

    return result;
}

fn decimalToSnafu(total: i64, buf: []u8) []u8 {
    var i: usize = 0;

    std.debug.assert(total > 0);

    while (true) {
        var offset: usize = 0;

        for (0..i) |n| {
            offset += 2 * std.math.pow(usize, 5, n);
        }

        buf[i] = switch (((offset + @as(usize, @intCast(total))) / std.math.pow(usize, 5, i)) % 5) {
            0 => '0',
            1 => '1',
            2 => '2',
            3 => '=',
            4 => '-',
            else => unreachable,
        };

        i += 1;

        if (i > 20) break;
    }

    std.mem.reverse(u8, buf[0..i]);

    var start: usize = 0;

    while (buf[start] == '0') : (start += 1) {}

    std.debug.assert(total == snafuToDecimal(buf[start..i]));

    return buf[start..i];
}

fn snafuToDecimal(snafu: []const u8) i64 {
    var n: i64 = 0;

    for (0.., snafu) |i, c| {
        n += @as(i64, switch (c) {
            '2' => 2,
            '1' => 1,
            '0' => 0,
            '-' => -1,
            '=' => -2,
            else => unreachable,
        }) * std.math.pow(i64, 5, @as(i32, @intCast(snafu.len - i)) - 1);
    }

    return n;
}

test {
    try std.testing.expectEqual(1747, snafuToDecimal("1=-0-2"));
    try std.testing.expectEqual(906, snafuToDecimal("12111"));
    try std.testing.expectEqual(198, snafuToDecimal("2=0="));
    try std.testing.expectEqual(11, snafuToDecimal("21"));
    try std.testing.expectEqual(201, snafuToDecimal("2=01"));
    try std.testing.expectEqual(31, snafuToDecimal("111"));
    try std.testing.expectEqual(1257, snafuToDecimal("20012"));
    try std.testing.expectEqual(32, snafuToDecimal("112"));
    try std.testing.expectEqual(353, snafuToDecimal("1=-1="));
    try std.testing.expectEqual(107, snafuToDecimal("1-12"));
    try std.testing.expectEqual(7, snafuToDecimal("12"));
    try std.testing.expectEqual(3, snafuToDecimal("1="));
    try std.testing.expectEqual(37, snafuToDecimal("122"));
}

test {
    var buf: [100]u8 = undefined;
    try std.testing.expectEqualSlices(u8, "1", decimalToSnafu(1, &buf));
    try std.testing.expectEqualSlices(u8, "2", decimalToSnafu(2, &buf));
    try std.testing.expectEqualSlices(u8, "1=", decimalToSnafu(3, &buf));
    try std.testing.expectEqualSlices(u8, "1-", decimalToSnafu(4, &buf));
    try std.testing.expectEqualSlices(u8, "10", decimalToSnafu(5, &buf));
    try std.testing.expectEqualSlices(u8, "11", decimalToSnafu(6, &buf));
    try std.testing.expectEqualSlices(u8, "12", decimalToSnafu(7, &buf));
    try std.testing.expectEqualSlices(u8, "2=", decimalToSnafu(8, &buf));
    try std.testing.expectEqualSlices(u8, "2-", decimalToSnafu(9, &buf));
    try std.testing.expectEqualSlices(u8, "20", decimalToSnafu(10, &buf));
    try std.testing.expectEqualSlices(u8, "1=0", decimalToSnafu(15, &buf));
    try std.testing.expectEqualSlices(u8, "1-0", decimalToSnafu(20, &buf));
    try std.testing.expectEqualSlices(u8, "1=11-2", decimalToSnafu(2022, &buf));
    try std.testing.expectEqualSlices(u8, "1-0---0", decimalToSnafu(12345, &buf));
    try std.testing.expectEqualSlices(u8, "1121-1110-1=0", decimalToSnafu(314159265, &buf));
}

test {
    var buf: [100]u8 = undefined;
    try std.testing.expectEqualSlices(u8, "2=-1=0", solve(
        \\1=-0-2
        \\12111
        \\2=0=
        \\21
        \\2=01
        \\111
        \\20012
        \\112
        \\1=-1=
        \\1-12
        \\12
        \\1=
        \\122
        \\
    , &buf));
}

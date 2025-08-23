const std = @import("std");

pub fn main() !void {
    var debug_allocator: std.heap.DebugAllocator(.{}) = .init;
    const gpa = debug_allocator.allocator();
    std.debug.print("{d}\n", .{try solve(gpa, @embedFile("day_20.txt"), 1, 1)});
    std.debug.print("{d}\n", .{try solve(gpa, @embedFile("day_20.txt"), 10, 811589153)});
}

const NumberCycle = struct {
    data: i64,
    node: std.DoublyLinkedList.Node = .{},
};

fn solve(gpa: std.mem.Allocator, input: []const u8, num_mixes: usize, decryption_key: i64) !i64 {
    var lines = std.mem.tokenizeScalar(u8, input, '\n');

    var numbers: std.DoublyLinkedList = .{};

    while (lines.next()) |line| {
        const n = try std.fmt.parseInt(i64, line, 10) * decryption_key;

        var number = try gpa.create(NumberCycle);

        number.* = .{ .data = n };

        numbers.append(&number.node);
    }

    var all_nodes: std.ArrayList(*std.DoublyLinkedList.Node) = try .initCapacity(gpa, numbers.len());
    defer all_nodes.deinit(gpa);

    {
        var it = numbers.first;

        for (0..numbers.len()) |_| {
            all_nodes.appendAssumeCapacity(it.?);
            it = it.?.next;
        }
    }

    defer {
        for (all_nodes.items) |node| {
            const l: *NumberCycle = @fieldParentPtr("node", node);
            gpa.destroy(l);
        }
    }

    for (0..num_mixes) |_| {
        mix(&numbers, all_nodes.items);
    }

    // first find zero
    var it = numbers.first;

    while (true) {
        const node = (it orelse numbers.first).?;

        const l: *NumberCycle = @fieldParentPtr("node", node);
        if (l.data == 0) break;

        it = node.next;
    }

    // then go forward 1000 times, 3 times
    var result: i64 = 0;

    for (0..3) |_| {
        for (0..1000) |_| {
            it = (it orelse numbers.first).?.next;
        }

        const l: *NumberCycle = @fieldParentPtr("node", it.?);
        result += l.data;
    }

    return result;
}

fn mix(numbers: *std.DoublyLinkedList, all_nodes: []*std.DoublyLinkedList.Node) void {
    for (all_nodes) |node| {
        const l: *NumberCycle = @fieldParentPtr("node", node);

        if (l.data == 0) continue;

        numbers.remove(node);

        var cursor = node;

        if (l.data > 0) {
            for (0..@as(usize, @intCast(l.data)) % (all_nodes.len - 1)) |_| {
                cursor = if (cursor.next) |n| n else numbers.first.?;
            }
        } else {
            for (0..@as(usize, @intCast(-l.data + 1)) % (all_nodes.len - 1)) |_| {
                cursor = if (cursor.prev) |p| p else numbers.last.?;
            }
        }

        numbers.insertAfter(cursor, node);
    }
}

test {
    try std.testing.expectEqual(3, try solve(std.testing.allocator,
        \\1
        \\2
        \\-3
        \\3
        \\-2
        \\0
        \\4
        \\
    , 1, 1));
}

test {
    try std.testing.expectEqual(1623178306, try solve(std.testing.allocator,
        \\1
        \\2
        \\-3
        \\3
        \\-2
        \\0
        \\4
        \\
    , 10, 811589153));
}

const std = @import("std");
const net = std.Io.net;
const Io = std.Io;

pub fn main() !void {
    // allocator
    var gpa = std.heap.DebugAllocator(.{}){};
    defer _ = gpa.deinit();
    const allocator = gpa.allocator();
    var threaded = Io.Threaded.init(allocator, .{});
    const io = threaded.io();

    // tcp stream
    const host = try net.HostName.init("irc.freenode.net");
    const stream = try host.connect(io, 6667, .{ .mode = .stream });
    defer stream.close(io);

    // write to stream on connect
    var buf: [512]u8 = undefined;
    var writer = stream.writer(io, &buf);
    try writer.interface.writeAll("NICK neonvoid\r\n");
    try writer.interface.writeAll("USER neonvoid 0 * :neonvoid\r\n");
    try writer.interface.flush();

    var rbuf: [512]u8 = undefined;
    var reader = stream.reader(io, &rbuf);
    while (try reader.interface.takeDelimiter('\n')) |line| {
        std.debug.print("{s}\n", .{line});
    }
}

const std = @import("std");
const vxfw = @import("vaxis").vxfw;
const TerminalGui = @import("./gui/gui.zig").TerminalGui;

pub fn main() !void {
    var gpa = std.heap.GeneralPurposeAllocator(.{}){};
    defer _ = gpa.deinit();

    const allocator = gpa.allocator();

    var app = try vxfw.App.init(allocator);
    defer app.deinit();

    const terminal_gui = try TerminalGui.create(allocator);
    defer terminal_gui.deinit(allocator);

    try app.run(terminal_gui.widget(), .{});
}
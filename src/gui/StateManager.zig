const std = @import("std");
const vxfw = @import("vaxis").vxfw;

pub const StateManager = struct {

    testu: usize = 44,

    pub fn create(allocator: std.mem.Allocator) std.mem.Allocator.Error!*StateManager {
        const state_manager = try allocator.create(StateManager);
        state_manager.* = .{};
        return state_manager;
    }

    pub fn process_event(self: *StateManager, event_ctx: *vxfw.EventContext, event: vxfw.Event) void {
        switch (event) {
            .key_press => |key| {
                if (key.matches('c', .{ .ctrl = true })) {
                    event_ctx.quit = true;
                    return;
                }else if (key.matches(57352, .{})){
                    self.testu = 1;
                }
                event_ctx.consumeAndRedraw();
            },
            else => {},
        }
    }
};
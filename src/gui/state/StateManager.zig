const std = @import("std");
const vxfw = @import("vaxis").vxfw;
const ContainerStore = @import("ContainerStore.zig").ContainerStore;

pub const StateManager = struct {

    container_store: *ContainerStore,

    pub fn create(allocator: std.mem.Allocator) std.mem.Allocator.Error!*StateManager {
        const state_manager = try allocator.create(StateManager);
        state_manager.* = .{
            .container_store = try ContainerStore.init(allocator)
        };
        return state_manager;
    }

    pub fn deinit(self: *StateManager, allocator: std.mem.Allocator) void {
        self.container_store.deinit(allocator);
        allocator.destroy(self);
    }

    pub fn process_event(self: *StateManager, event_ctx: *vxfw.EventContext, event: vxfw.Event) !void {
        switch (event) {
            .key_press => |key| {
                if (key.matches('c', .{ .ctrl = true })) {
                    event_ctx.quit = true;
                    return;
                }else if (key.matches(57352, .{})){
                    try self.container_store.refetch();
                }
                event_ctx.consumeAndRedraw();
            },
            else => {},
        }
    }
};
const std = @import("std");
const vxfw = @import("vaxis").vxfw;
const StateManager = @import("statemanager.zig").StateManager;

pub const TerminalGui = struct {

    state_manager: *StateManager,

    pub fn create(allocator: std.mem.Allocator) std.mem.Allocator.Error!*TerminalGui {
        const termina_gui = try allocator.create(TerminalGui);
        termina_gui.* = .{ .state_manager = try StateManager.create(allocator) };
        return termina_gui;
    }

    pub fn deinit(self: *TerminalGui, allocator: std.mem.Allocator) void {
        allocator.destroy(self.state_manager);
        allocator.destroy(self);
    }

    pub fn widget(self: *TerminalGui) vxfw.Widget {
        return .{
            .userdata = self,
            .eventHandler = TerminalGui.typeErasedEventHandler,
            .drawFn = TerminalGui.typeErasedDrawFn,
        };
    }

    fn typeErasedEventHandler(self_ptr: *anyopaque, event_ctx: *vxfw.EventContext, event: vxfw.Event) anyerror!void {
        const self: *TerminalGui = @ptrCast(@alignCast(self_ptr));
        self.state_manager.process_event(event_ctx, event);
    }

    fn typeErasedDrawFn(ptr: *anyopaque, ctx: vxfw.DrawContext) std.mem.Allocator.Error!vxfw.Surface {
        const self: *TerminalGui = @ptrCast(@alignCast(ptr));
        const max_size = ctx.max.size();

        const count_text = try std.fmt.allocPrint(ctx.arena, "{d}", .{self.state_manager.testu});
        const text: vxfw.Text = .{ .text = count_text };

        const counter_border: vxfw.Border = .{
            .child = text.widget()
        };

        const button_border: vxfw.Border = .{
            .child = text.widget()
        };

        const flex: vxfw.FlexRow = .{
            .children = &.{
                .{ .widget = counter_border.widget(), .flex = 2},
                .{ .widget = button_border.widget(), .flex = 1},
            }
        };

        const flex_child: vxfw.SubSurface = .{
            .origin = .{ .row = 0, .col = 0 },
            .surface = try flex.draw(ctx),
        };

        const children = try ctx.arena.alloc(vxfw.SubSurface, 1);
        children[0] = flex_child;

        return .{
            .size = max_size,
            .widget = self.widget(),
            .buffer = &.{},
            .children = children,
        };
    }
};
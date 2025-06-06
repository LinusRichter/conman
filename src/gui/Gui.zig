const std = @import("std");
const vaxis = @import("vaxis");
const vxfw = vaxis.vxfw;
const StateManager = @import("./state/statemanager.zig").StateManager;
const ContainerList = @import("./components/ContainerList.zig").ContainerList;

pub const TerminalGui = struct {

    state_manager: *StateManager,

    //components:
    container_list: *ContainerList,

    pub fn create(allocator: std.mem.Allocator) std.mem.Allocator.Error!*TerminalGui {
        const termina_gui = try allocator.create(TerminalGui);
        const state_manager = try StateManager.create(allocator);
        const container_list = try ContainerList.create(allocator, state_manager.container_store);
        termina_gui.* = .{
            .state_manager = state_manager,
            .container_list = container_list,
        };
        return termina_gui;
    }

    pub fn deinit(self: *TerminalGui, allocator: std.mem.Allocator) void {
        self.state_manager.deinit(allocator);
        self.container_list.deinit(allocator);
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
        try self.state_manager.process_event(event_ctx, event);
    }

    fn typeErasedDrawFn(ptr: *anyopaque, ctx: vxfw.DrawContext) std.mem.Allocator.Error!vxfw.Surface {
        const self: *TerminalGui = @ptrCast(@alignCast(ptr));
        const max_size = ctx.max.size();

        //const count_text = try std.fmt.allocPrint(ctx.arena, "{d}", .{self.state_manager.testu});
        const entity_column_string = try std.fmt.allocPrint(ctx.arena, "{s}", .{"Choose Entity"});
        //const result_column_string = try std.fmt.allocPrint(ctx.arena, "{s}", .{"Choose Result"});
        const action_column_string = try std.fmt.allocPrint(ctx.arena, "{s}", .{"Choose Action"});

        const entity_column_text: vxfw.Text = .{ .text = entity_column_string };
        //const result_column_text: vxfw.Text = .{ .text = result_column_string };
        const action_column_text: vxfw.Text = .{ .text = action_column_string };

        const entity_column: vxfw.Border = .{ .child = entity_column_text.widget() };
        const result_column: vxfw.Border = .{ .child = self.container_list.widget() };
        const action_column: vxfw.Border = .{ .child = action_column_text.widget() };

        const flex_row: vxfw.FlexRow = .{
            .children = &.{
                .{ .widget = entity_column.widget(), .flex = 1},
                .{ .widget = result_column.widget(), .flex = 2},
                .{ .widget = action_column.widget(), .flex = 1},
            }
        };

        const flex_surface: vxfw.SubSurface = .{
            .origin = .{ .row = 0, .col = 0 },
            .surface = try flex_row.draw(ctx),
        };

        const children = try ctx.arena.alloc(vxfw.SubSurface, 1);
        children[0] = flex_surface;

        return .{
            .size = max_size,
            .widget = self.widget(),
            .buffer = &.{},
            .children = children,
        };
    }
};
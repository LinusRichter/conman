const std = @import("std");
const vaxis = @import("vaxis");
const vxfw = vaxis.vxfw;
const ContainerStore = @import("../state/ContainerStore.zig").ContainerStore;

pub const ContainerList = struct {

    container_store: *ContainerStore,

    pub fn create(allocator: std.mem.Allocator, container_store: *ContainerStore) std.mem.Allocator.Error!*ContainerList {
        const container_list = try allocator.create(ContainerList);
        container_list.* = .{
            .container_store = container_store
        };
        return container_list;
    }

    pub fn deinit(self: *ContainerList, allocator: std.mem.Allocator) void {
        allocator.destroy(self);
    }

    pub fn widget(self: *ContainerList) vxfw.Widget {
        return .{
            .userdata = self,
            .drawFn = ContainerList.typeErasedDrawFn,
        };
    }

    fn typeErasedDrawFn(ptr: *anyopaque, ctx: vxfw.DrawContext) std.mem.Allocator.Error!vxfw.Surface {
        const self: *ContainerList = @ptrCast(@alignCast(ptr));

        const container_text: vxfw.Text = .{ .text = self.container_store.data.items[0] };

        const subsurface: vxfw.SubSurface = .{
            .origin = .{ .row = 0, .col = 0 },
            .surface = try container_text.draw(ctx),
        };

        const children = try ctx.arena.alloc(vxfw.SubSurface, 1);
        children[0] = subsurface;

        return .{
            .size = ctx.max.size(),
            .widget = self.widget(),
            .buffer = &.{},
            .children = children,
        };
    }
};
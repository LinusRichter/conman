const std = @import("std");
const vaxis = @import("vaxis");
const vxfw = vaxis.vxfw;

pub const ContainerList = struct {

    pub fn widget(self: *ContainerList) vxfw.Widget {
        return .{
            .userdata = self,
            .drawFn = ContainerList.typeErasedDrawFn,
        };
    }

    fn typeErasedDrawFn(ptr: *anyopaque, ctx: vxfw.DrawContext) std.mem.Allocator.Error!vxfw.Surface {
        const self: *ContainerList = @ptrCast(@alignCast(ptr));
        return .{
            .size = ctx.max.size(),
            .widget = self.widget(),
            .buffer = &.{},
            .children = children,
        };
    }
};
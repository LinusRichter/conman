const std = @import("std");

pub const ContainerStore = struct {
    data: std.ArrayList([]const u8),

    pub fn init(allocator: std.mem.Allocator) std.mem.Allocator.Error!*ContainerStore {
        const container_store = try allocator.create(ContainerStore);
        container_store.* = .{ .data = std.ArrayList([]const u8).init(allocator) };
        try container_store.data.append("TestContainer");
        return container_store;
    }

    pub fn deinit(self: *ContainerStore, allocator: std.mem.Allocator) void {
        self.data.deinit();
        allocator.destroy(self);
    }

    pub fn refetch(self: *ContainerStore) !void {
        try self.data.append("AnotherTestContaine");
    }
};
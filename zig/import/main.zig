const std = @import("std");
const user = @import("models/users.zig");

const User = user.User;
const MAX_POWER = user.MAX_POWER;

pub fn main() void {
    std.debug.print("The User is {} and the pwer is {d}", .{User, MAX_POWER});
}


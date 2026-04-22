const std = @import("std");

pub const User = struct {
    power: u64 = 0,
    name: []const u8,
    
    pub const SUPER_POWER = 9000;

    pub fn init(name: []const u8, power: u64) User{
        return User{
            .name = name,
            .power = power,
        };
    }

    pub fn diagnose(user: User) void {
        if (user.power >= SUPER_POWER){
            std.debug.print("it's over {d}!!!", .{SUPER_POWER});
        } else {
            std.debug.print("{s}'s power level is only {d}\n", .{user.name, user.power});
        }
    }
};

pub fn main() void{
    
    const vegeta: User = User.init("Vegeta", 8999);

    const goku = User{
        .power = 9001,
        .name = "Goku",
    };

    User.diagnose(goku);
    User.diagnose(vegeta);
}

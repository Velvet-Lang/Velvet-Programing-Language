const std = @import("std");

pub fn build(b: *std.Build) void {
    const exe = b.addExecutable(.{
        .name = "velvet-runtime",
        .root_source_file = .{ .path = "src/runtime.zig" },
        .target = b.standardTargetOptions(.{}),
        .optimize = b.standardOptimizeOption(.{}),
    });

    // Link multiple .o files
    exe.addObjectFile(.{ .path = "file1.o" });
    exe.addObjectFile(.{ .path = "file2.o" });

    // Embed resources (e.g., images as bytes)
    const embed = b.addEmbeddedFile("resources", .{ .path = "assets/image.png" });
    exe.addObject(embed);

    b.installArtifact(exe);

    // Run command
    const run_cmd = b.addRunArtifact(exe);
    b.step("run", "Run the app").dependOn(&run_cmd.step);
}

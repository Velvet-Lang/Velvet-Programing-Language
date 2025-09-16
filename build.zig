const std = @import("std");

const VelvetError = error{
    InvalidObjectFile,
    ResourceEmbedFail,
    LinkError,
};

pub fn build(b: *std.Build) void {
    const target = b.standardTargetOptions(.{});
    const optimize = b.standardOptimizeOption(.{});

    const exe = b.addExecutable(.{
        .name = "velvet-runtime",
        .root_source_file = b.path("src/runtime.zig"),
        .target = target,
        .optimize = optimize,
    });

    // Link all .o files from build dir
    const o_files = getObjectFiles(b.allocator) catch |err| {
        std.log.err("Failed to find .o files: {}", .{err});
        return;
    };
    for (o_files.items) |o_file| {
        exe.addObjectFile(b.path(o_file));
    }

    // Embed resources (images, configs, etc.)
    const resources_dir = "assets/";
    const resources = getResources(b.allocator, resources_dir) catch |err| {
        std.log.err("Failed to embed resources: {}", .{err});
        return;
    };
    for (resources.items) |res| {
        const embed = b.addEmbeddedFile(res, .{ .path = res });
        exe.addObject(embed);
    }

    // Add FFI links (e.g., libpython, QuickJS for embeds)
    exe.linkLibC();
    exe.linkSystemLibrary("python3.10"); // Stub for Python FFI
    // Add more links as needed (e.g., for JS: quickjs)

    b.installArtifact(exe);

    // Run command with hot reload option
    const run_cmd = b.addRunArtifact(exe);
    run_cmd.step.dependOn(b.getInstallStep());
    if (b.args) |args| {
        run_cmd.addArgs(args);
    }

    const run_step = b.step("run", "Run the Velvet runtime");
    run_step.dependOn(&run_cmd.step);

    // Hot reload step (watch for changes)
    const hot_reload_step = b.step("hot-reload", "Run with hot reload");
    const watch_cmd = b.addSystemCommand(&[_][]const u8{
        "fswatch", "-o", "src/", "build/" // Use fswatch or similar for watching
    });
    const rebuild_cmd = b.addSystemCommand(&[_][]const u8{"zig", "build", "run"});
    hot_reload_step.dependOn(&watch_cmd.step);
    hot_reload_step.dependOn(&rebuild_cmd.step);

    // Test command
    const test_step = b.step("test", "Run tests");
    const unit_tests = b.addTest(.{
        .root_source_file = b.path("src/runtime.zig"),
        .target = target,
        .optimize = optimize,
    });
    const run_tests = b.addRunArtifact(unit_tests);
    test_step.dependOn(&run_tests.step);

    // Cross-compilation support (stub for Windows/macOS)
    const cross_win = b.addExecutable(.{
        .name = "velvet-runtime-win",
        .root_source_file = b.path("src/runtime.zig"),
        .target = .{ .cpu_arch = .x86_64, .os_tag = .windows },
        .optimize = optimize,
    });
    b.installArtifact(cross_win);
}

fn getObjectFiles(allocator: std.mem.Allocator) error{OutOfMemory}!std.ArrayList([]const u8) {
    var files = std.ArrayList([]const u8).init(allocator);
    var dir = std.fs.cwd().openDir("build", .{ .iterate = true }) catch return error.NoBuildDir;
    defer dir.close();
    var iterator = dir.iterate();
    while (try iterator.next()) |entry| {
        if (std.mem.endsWith(u8, entry.name, ".o")) {
            try files.append(try allocator.dupe(u8, entry.name));
        }
    }
    return files;
}

fn getResources(allocator: std.mem.Allocator, dir_path: []const u8) error{OutOfMemory}!std.ArrayList([]const u8) {
    var files = std.ArrayList([]const u8).init(allocator);
    var dir = std.fs.cwd().openDir(dir_path, .{ .iterate = true }) catch return error.NoAssetsDir;
    defer dir.close();
    var iterator = dir.iterate();
    while (try iterator.next()) |entry| {
        try files.append(try allocator.dupe(u8, entry.name));
    }
    return files;
}

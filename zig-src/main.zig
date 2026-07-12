const std = @import("std");
const process = std.process;
const heap = std.heap;
const ansi = @import("ansi.zig");
const Ansi = ansi.Ansi;
const utils = @import("utils.zig");

const cpu = @cImport({
    @cInclude("cpu.h");
});

const gpu = @cImport({
    @cInclude("gpu.h");
});

const pci = @cImport({
    @cInclude("pci.h");
});

const system = @cImport({
    @cInclude("system.h");
});

pub fn main() !void {
    const stdout = std.io.getStdOut().writer();
    const allocator = heap.page_allocator;

    try printSystem(allocator, stdout);
    try printMachine(allocator, stdout);
    try printCPU(allocator, stdout);
    try printGraphics(allocator, stdout);
    try printAudio(allocator, stdout);
    try printNetwork(allocator, stdout);
    try getNetworkDevices(allocator, stdout);
    try getInterfaceInfo(allocator, stdout);
    try getNetworkServices(allocator, stdout);
    try printDrives(allocator, stdout);
    try printPartition(allocator, stdout);
    try printSwap(allocator, stdout);
    try printSensors(allocator, stdout);
    try printInfo(allocator, stdout);

    //var pinfo: pci.PCIInfo = undefined;
    //_ = pci.get_pci_info_full(&pinfo);
    //std.debug.print("PCI SUBSYSVENDOR: {s}\n", .{pinfo.subvendor});

    // Print colored "Network:" label
    //try stdout.print("{s}Network:{s}\n", .{ Ansi.blue_bold_str, Ansi.reset });

    // Get network interface information using lspci and ip commands

}

const WriterType = @TypeOf(std.io.getStdOut().writer());

fn printSystem(allocator: std.mem.Allocator, writer: WriterType) !void {
    _ = allocator;
    try writer.print("{s}System:{s}\n", .{ Ansi.blue_bold_str, Ansi.reset });

    var cinfo: system.SystemInfo = undefined;
    _ = system.get_system_info(&cinfo);

    try writer.print("  {s}Host:{s} {s}", .{ Ansi.blue_bold_str, Ansi.reset, utils.nullTerminateCString(&cinfo.nodename) });

    try writer.print(" {s}Kernel:{s} {s} ", .{ Ansi.blue_bold_str, Ansi.reset, utils.nullTerminateCString(&cinfo.release) });

    try writer.print(" {s}Arch:{s} {s}\n", .{ Ansi.blue_bold_str, Ansi.reset, utils.nullTerminateCString(&cinfo.machine) });

    try writer.print("  {s}compiler:{s} {s}", .{ Ansi.blue_bold_str, Ansi.reset, utils.nullTerminateCString(&cinfo.compiler_name) });

    try writer.print(" {s}clocksource:{s} {s}", .{ Ansi.blue_bold_str, Ansi.reset, utils.nullTerminateCString(&cinfo.clc_current) });

    try writer.print(" {s}avail:{s} {s}\n", .{ Ansi.blue_bold_str, Ansi.reset, utils.nullTerminateCString(&cinfo.clc_avail) });

    try writer.print("  {s}parameters:{s} {s}", .{ Ansi.blue_bold_str, Ansi.reset, utils.nullTerminateCString(&cinfo.params) });
    return;
}

fn printMachine(allocator: std.mem.Allocator, writer: WriterType) !void {
    _ = allocator;
    try writer.print("{s}Machine:{s}\n", .{ Ansi.blue_bold_str, Ansi.reset });
    return;
}

fn printCPU(allocator: std.mem.Allocator, writer: WriterType) !void {
    _ = allocator;
    try writer.print("{s}CPU:{s}\n", .{ Ansi.blue_bold_str, Ansi.reset });
    try writer.print("  {s}Info:{s} ", .{ Ansi.blue_bold_str, Ansi.reset });

    var cinfo: cpu.CPUInfo = undefined;
    _ = cpu.get_cpu_info_full(&cinfo);

    const cpu_model = std.mem.trim(u8, std.mem.span(cinfo.brand), " ");

    try writer.print("{s}model:{s} {s} ", .{ Ansi.blue_bold_str, Ansi.reset, cpu_model });

    // AuthenticAMD or GenuineIntel
    const original_vendor = std.mem.trim(u8, std.mem.span(cinfo.vendor), " ");
    var abbreviated_vendor: []const u8 = "Unknown";
    if (std.mem.eql(u8, original_vendor, "AuthenticAMD")) {
        abbreviated_vendor = "AMD";
    }

    if (std.mem.eql(u8, original_vendor, "GenuineIntel")) {
        abbreviated_vendor = "Intel";
    }

    try writer.print("{s}vendor:{s} {s}\n", .{ Ansi.blue_bold_str, Ansi.reset, abbreviated_vendor });
    try writer.print("  {s}Cores:{s} {d} ", .{ Ansi.blue_bold_str, Ansi.reset, cinfo.cores });
    try writer.print("{s}Threads:{s} {d}", .{ Ansi.blue_bold_str, Ansi.reset, cinfo.threads });
    try writer.print(" {s}Stepping:{s} {d}\n", .{ Ansi.blue_bold_str, Ansi.reset, cinfo.stepping });
    var buffer: [16]u8 = undefined;
    const model_id_in_hex = try std.fmt.bufPrint(&buffer, "0x{x}", .{@as(u32, @intCast(cinfo.model))});
    try writer.print("  {s}Model ID:{s} {d} {s}", .{ Ansi.blue_bold_str, Ansi.reset, cinfo.model, model_id_in_hex });

    const family_id_in_hex = try std.fmt.bufPrint(&buffer, "0x{x}", .{@as(u32, @intCast(cinfo.family))});
    try writer.print(" {s}Family:{s} {d} {s}\n", .{ Ansi.blue_bold_str, Ansi.reset, cinfo.family, family_id_in_hex });

    _ = cpu.get_cpu_free(&cinfo);
    return;
}

fn printGraphics(allocator: std.mem.Allocator, writer: WriterType) !void {
    try writer.print("{s}Graphics:{s}\n", .{ Ansi.blue_bold_str, Ansi.reset });

    // In the future GPUInfo will be a list of GPUs instead of GPU0
    _ = allocator;
    //const GPUDevicesType = struct {
    //    name: []const u8,
    //   driver_version: []const u8,
    // driver: []const u8,
    //vendor: []const u8,
    //};

    //var gpu_devices: std.ArrayList(GPUDevicesType) = std.ArrayList(GPUDevicesType).init(allocator);
    //defer gpu_devices.deinit();

    var ginfo: gpu.GPUInfo = std.mem.zeroInit(gpu.GPUInfo, .{});
    _ = gpu.get_full_gpu_info(&ginfo);
    try writer.print("  {s}Device-1:{s} ", .{ Ansi.blue_bold_str, Ansi.reset });
    try writer.print("{s}", .{ginfo.name});
    try writer.print(" {s}Driver:{s} {s}", .{ Ansi.blue_bold_str, Ansi.reset, "Unknown" });
    try writer.print(" {s}v:{s} {s}\n", .{ Ansi.blue_bold_str, Ansi.reset, ginfo.version });

    try writer.print("  {s}Vendor:{s} {s}\n", .{ Ansi.blue_bold_str, Ansi.reset, ginfo.vendor });

    return;
}

fn printAudio(allocator: std.mem.Allocator, writer: WriterType) !void {
    _ = allocator;
    try writer.print("{s}Audio:{s}\n", .{ Ansi.blue_bold_str, Ansi.reset });
    return;
}

fn printNetwork(allocator: std.mem.Allocator, writer: WriterType) !void {
    _ = allocator;
    try writer.print("{s}Network:{s}\n", .{ Ansi.blue_bold_str, Ansi.reset });
    return;
}

fn printDrives(allocator: std.mem.Allocator, writer: WriterType) !void {
    _ = allocator;
    try writer.print("{s}Drives:{s}\n", .{ Ansi.blue_bold_str, Ansi.reset });
    return;
}

fn printPartition(allocator: std.mem.Allocator, writer: WriterType) !void {
    _ = allocator;
    try writer.print("{s}Partition:{s}\n", .{ Ansi.blue_bold_str, Ansi.reset });
    return;
}

fn printSwap(allocator: std.mem.Allocator, writer: WriterType) !void {
    _ = allocator;
    try writer.print("{s}Swap:{s}\n", .{ Ansi.blue_bold_str, Ansi.reset });
    return;
}

fn printSensors(allocator: std.mem.Allocator, writer: WriterType) !void {
    _ = allocator;
    try writer.print("{s}Sensors:{s}\n", .{ Ansi.blue_bold_str, Ansi.reset });
    return;
}

fn printInfo(allocator: std.mem.Allocator, writer: WriterType) !void {
    _ = allocator;
    try writer.print("{s}Info:{s}\n", .{ Ansi.blue_bold_str, Ansi.reset });
    return;
}

fn getNetworkDevices(allocator: std.mem.Allocator, writer: anytype) !void {
    // Use lspci to get network device information
    const lspci_args = &[_][]const u8{ "lspci", "-v" };
    var lspci_process = process.Child.init(lspci_args, allocator);
    lspci_process.stdout_behavior = .Pipe;
    lspci_process.stderr_behavior = .Ignore;

    lspci_process.spawn() catch {
        return;
    };

    var buffer: [8192]u8 = undefined;
    const stdout = lspci_process.stdout.?;

    const bytesRead = stdout.readAll(buffer[0..]) catch {
        _ = lspci_process.wait() catch {};
        return;
    };

    _ = lspci_process.wait() catch {};

    if (bytesRead > 0) {
        const content = buffer[0..bytesRead];
        try parseLspciVOutput(writer, content);
    }
}

fn parseLspciVOutput(writer: anytype, content: []const u8) !void {
    var lines = std.mem.splitScalar(u8, content, '\n');
    var device_count: usize = 0;
    var current_device_id: ?[]const u8 = null;

    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " ");
        if (trimmed.len == 0) continue;

        if (std.ascii.isDigit(trimmed[0]) and
            (std.mem.indexOf(u8, trimmed, "Ethernet controller") != null or
                std.mem.indexOf(u8, trimmed, "Network controller") != null))
        {
            device_count += 1;
            current_device_id = trimmed;

            var device_name: []const u8 = trimmed;
            if (std.mem.indexOf(u8, trimmed, "Ethernet controller:") != null) {
                const pos = std.mem.indexOf(u8, trimmed, "Ethernet controller:").? + 20;
                device_name = std.mem.trim(u8, trimmed[pos..], " ");
            } else if (std.mem.indexOf(u8, trimmed, "Network controller:") != null) {
                const pos = std.mem.indexOf(u8, trimmed, "Network controller:").? + 19;
                device_name = std.mem.trim(u8, trimmed[pos..], " ");
            } else {
                // Extract everything after the device ID (first part)
                var parts = std.mem.splitScalar(u8, trimmed, ' ');
                _ = parts.next(); // Skip device ID
                if (parts.next()) |_| {
                    // Find the actual device name part
                    if (std.mem.indexOf(u8, trimmed, ": ") != null) {
                        const colon_pos = std.mem.indexOf(u8, trimmed, ": ").? + 2;
                        device_name = std.mem.trim(u8, trimmed[colon_pos..], " ");
                    }
                }
            }

            try writer.print("  {s}Device-{d}:{s} {s}\n", .{ Ansi.green_bold_str, device_count, Ansi.reset, device_name });
        }
        // Parse additional device details
        else if (current_device_id != null and device_count > 0) {
            // This would parse vendor, driver, PCIe info, etc.
            // For now, we'll add placeholder parsing logic
            if (std.mem.indexOf(u8, trimmed, "Subsystem:") != null) {
                // We would extract vendor info here
            } else if (std.mem.indexOf(u8, trimmed, "Kernel driver in use:") != null) {
                // We would extract driver info here
            }
        }
    }
}

fn getInterfaceInfo(allocator: std.mem.Allocator, writer: anytype) !void {
    // Get interface information using ip command
    const ip_args = &[_][]const u8{ "ip", "addr", "show" };
    var ip_process = process.Child.init(ip_args, allocator);
    ip_process.stdout_behavior = .Pipe;
    ip_process.stderr_behavior = .Ignore;

    ip_process.spawn() catch {
        return;
    };

    var buffer: [8192]u8 = undefined;
    const stdout = ip_process.stdout.?;

    const bytesRead = stdout.readAll(buffer[0..]) catch {
        _ = ip_process.wait() catch {};
        return;
    };

    _ = ip_process.wait() catch {};

    if (bytesRead > 0) {
        const content = buffer[0..bytesRead];
        try parseInterfaceInfo(writer, content);
    }
}

fn parseInterfaceInfo(writer: anytype, content: []const u8) !void {
    var lines = std.mem.splitScalar(u8, content, '\n');
    var interface_count: usize = 0;

    while (lines.next()) |line| {
        const trimmed = std.mem.trim(u8, line, " ");
        if (trimmed.len == 0) continue;

        // Look for interface definitions (lines starting with number)
        if (std.ascii.isDigit(trimmed[0])) {
            var parts = std.mem.splitScalar(u8, trimmed, ':');
            _ = parts.next(); // skip the number
            if (parts.next()) |iface_name| {
                const clean_name = std.mem.trim(u8, iface_name, " ");
                if (std.mem.indexOf(u8, clean_name, "lo") == 0) continue; // skip loopback

                interface_count += 1;
                if (interface_count == 1) {
                    try writer.print("  {s}IF-1:{s} {s}", .{ Ansi.blue_bold_str, Ansi.reset, clean_name });
                } else {
                    try writer.print("\n  {s}IF-{d}:{s} {s}", .{ Ansi.blue_bold_str, interface_count, Ansi.reset, clean_name });
                }
            }
        } else if (interface_count > 0 and interface_count == 1) {
            // Parse interface details for the first interface
            if (std.mem.indexOf(u8, trimmed, "state UP") != null) {
                try writer.print(" {s}state:{s} up", .{ Ansi.blue_bold_str, Ansi.reset });
            } else if (std.mem.indexOf(u8, trimmed, "state DOWN") != null) {
                try writer.print(" {s}state:{s} down", .{ Ansi.blue_bold_str, Ansi.reset });
            }
            // Parse MAC address
            else if (std.mem.indexOf(u8, trimmed, "link/ether") != null) {
                var link_parts = std.mem.splitScalar(u8, trimmed, ' ');
                _ = link_parts.next(); // "link/ether"
                if (link_parts.next()) |mac| {
                    const clean_mac = std.mem.trim(u8, mac, " ");
                    try writer.print(" {s}mac:{s} {s}", .{ Ansi.blue_bold_str, Ansi.reset, clean_mac });
                }
            }
        }
    }

    try writer.print("\n", .{});
}

fn getNetworkServices(allocator: std.mem.Allocator, writer: anytype) !void {
    try writer.print("  {s}Info:{s} ", .{ Ansi.blue_bold_str, Ansi.reset });

    var services = std.ArrayList([]const u8).init(allocator);
    defer {
        for (services.items) |service| {
            allocator.free(service);
        }
        services.deinit();
    }

    // Check if NetworkManager is running
    const nm_args = &[_][]const u8{ "systemctl", "is-active", "NetworkManager" };
    var nm_process = process.Child.init(nm_args, allocator);
    nm_process.stdout_behavior = .Pipe;
    nm_process.stderr_behavior = .Ignore;

    nm_process.spawn() catch {
        // If systemctl fails, try checking for running process
        const ps_args = &[_][]const u8{ "pgrep", "NetworkManager" };
        var ps_process = process.Child.init(ps_args, allocator);
        ps_process.stdout_behavior = .Pipe;
        ps_process.stderr_behavior = .Ignore;

        ps_process.spawn() catch {
            return;
        };

        var buffer: [128]u8 = undefined;
        const stdout = ps_process.stdout.?;

        const bytesRead = stdout.readAll(buffer[0..]) catch {
            _ = ps_process.wait() catch {};
            return;
        };

        if (bytesRead > 0) {
            const service_name = try allocator.dupe(u8, "NetworkManager");
            try services.append(service_name);
        }

        _ = ps_process.wait() catch {};
        return;
    };

    var buffer: [128]u8 = undefined;
    const stdout = nm_process.stdout.?;

    const bytesRead = stdout.readAll(buffer[0..]) catch {
        _ = nm_process.wait() catch {};
        return;
    };

    if (bytesRead > 0 and std.mem.indexOf(u8, buffer[0..bytesRead], "active") != null) {
        const service_name = try allocator.dupe(u8, "NetworkManager");
        try services.append(service_name);
    }

    _ = nm_process.wait() catch {};

    // Check if sshd is running
    const ssh_args = &[_][]const u8{ "systemctl", "is-active", "sshd" };
    var ssh_process = process.Child.init(ssh_args, allocator);
    ssh_process.stdout_behavior = .Pipe;
    ssh_process.stderr_behavior = .Ignore;

    ssh_process.spawn() catch {
        // Fallback to pgrep
        const ps_args = &[_][]const u8{ "pgrep", "sshd" };
        var ps_process = process.Child.init(ps_args, allocator);
        ps_process.stdout_behavior = .Pipe;
        ps_process.stderr_behavior = .Ignore;

        ps_process.spawn() catch {
            // Print what we have so far
            try printServices(services.items, writer);
            return;
        };

        const bytesRead2 = stdout.readAll(buffer[0..]) catch {
            _ = ps_process.wait() catch {};
            try printServices(services.items, writer);
            return;
        };

        if (bytesRead2 > 0) {
            const service_name = try allocator.dupe(u8, "sshd");
            try services.append(service_name);
        }

        _ = ps_process.wait() catch {};
        try printServices(services.items, writer);
        return;
    };

    const bytesRead2 = stdout.readAll(buffer[0..]) catch {
        _ = ssh_process.wait() catch {};
        try printServices(services.items, writer);
        return;
    };

    if (bytesRead2 > 0 and std.mem.indexOf(u8, buffer[0..bytesRead2], "active") != null) {
        const service_name = try allocator.dupe(u8, "sshd");
        try services.append(service_name);
    }

    _ = ssh_process.wait() catch {};
    try printServices(services.items, writer);
}

fn printServices(services: []const []const u8, writer: anytype) !void {
    try writer.print("{s}services:{s} ", .{ Ansi.blue_bold_str, Ansi.reset });
    for (services, 0..) |service, i| {
        if (i > 0) try writer.print(",", .{});
        try writer.print("{s}", .{service});
    }
    try writer.print("\n", .{});
}

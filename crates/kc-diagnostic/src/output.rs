use std::path::{Path, PathBuf};

use kc_domain::{
    Address, CapabilityProfile, DeviceDescriptor, HostOperationReadiness, MountPoint,
    ReadinessReport,
};

pub fn format_capabilities(capabilities: &CapabilityProfile) -> String {
    let values = capabilities
        .iter()
        .map(|capability| format!("{capability:?}"))
        .collect::<Vec<_>>();

    if values.is_empty() {
        "none".to_string()
    } else {
        values.join(", ")
    }
}

pub fn path_display_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| PathBuf::from(path).display().to_string())
}

pub fn print_assessment(
    mount: &MountPoint,
    descriptor: &DeviceDescriptor,
    readiness: &ReadinessReport,
    install_target: Option<&Address>,
    backup_target: Option<&Address>,
    install_root: Option<&Path>,
    backup_root: Option<&Path>,
) {
    println!("mount: {}", mount.root.display());
    println!("target: {}", descriptor.display_name);
    println!("support level: {:?}", descriptor.support_level);
    println!(
        "current readiness: {}",
        readiness_status_label(readiness.ready)
    );
    if readiness.blockers.is_empty() {
        println!("blockers: none");
    } else {
        println!("blockers:");
        for blocker in &readiness.blockers {
            println!("- {blocker}");
        }
    }

    match install_target {
        Some(address) => println!("install target: {}", format_address(address)),
        None => println!("install target: unavailable while current readiness is blocked"),
    }
    match backup_target {
        Some(address) => println!("backup target: {}", format_address(address)),
        None => println!("backup target: unavailable while current readiness is blocked"),
    }
    match install_root {
        Some(path) => println!("install root: {}", path.display()),
        None => println!("install root: unavailable while current readiness is blocked"),
    }
    match backup_root {
        Some(path) => println!("backup root: {}", path.display()),
        None => println!("backup root: unavailable while current readiness is blocked"),
    }
}

pub fn print_host_operation(name: &str, readiness: &HostOperationReadiness) {
    println!(
        "{name} automation readiness: {}",
        readiness_status_label(readiness.ready)
    );
    if readiness.blockers.is_empty() {
        println!("{name} blockers: none");
    } else {
        println!("{name} blockers:");
        for blocker in &readiness.blockers {
            println!("- {blocker}");
        }
    }
    if readiness.guidance.is_empty() {
        println!("{name} guidance: none");
    } else {
        println!("{name} guidance:");
        for guidance in &readiness.guidance {
            println!("- {guidance}");
        }
    }
}

pub fn print_usage() {
    println!("Usage:");
    println!("  kc-diagnostic foundation");
    println!("  kc-diagnostic probe <device-root>");
}

fn readiness_status_label(ready: bool) -> &'static str {
    if ready {
        "ready"
    } else {
        "blocked"
    }
}

fn format_address(address: &Address) -> String {
    match address {
        Address::LocalPath(path) => path.display().to_string(),
        Address::ScopedPath {
            transport,
            scope,
            relative_path,
        } => format!("{transport:?}:{scope}:{}", relative_path.display()),
        Address::Remote {
            transport,
            locator,
            path,
        } => format!("{transport:?}:{locator}:{path}"),
        Address::Logical { scheme, value } => format!("{scheme}:{value}"),
    }
}

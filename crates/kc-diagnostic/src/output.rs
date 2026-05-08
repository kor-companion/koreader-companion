use std::path::{Path, PathBuf};

use kc_device::DeviceAssessment;
use kc_domain::{Address, CapabilityProfile, HostOperationReadiness};

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

pub fn print_assessment(assessment: DeviceAssessment) {
    println!("mount: {}", assessment.mount.root.display());
    println!(
        "target: {} [{:?}]",
        assessment.descriptor.display_name, assessment.descriptor.support_level
    );
    println!("ready: {}", assessment.readiness.ready);
    if assessment.readiness.blockers.is_empty() {
        println!("blockers: none");
    } else {
        println!("blockers:");
        for blocker in assessment.readiness.blockers {
            println!("- {blocker}");
        }
    }
    println!(
        "install target: {}",
        format_address(&assessment.install_target)
    );
    println!(
        "backup target: {}",
        format_address(&assessment.backup_target)
    );
    println!("install root: {}", assessment.install_root.display());
    println!("backup root: {}", assessment.backup_root.display());
}

pub fn print_host_operation(name: &str, readiness: &HostOperationReadiness) {
    println!("{name} ready: {}", readiness.ready);
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

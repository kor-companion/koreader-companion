use std::env;
use std::path::{Path, PathBuf};

use kc_device::{assess_host_mounts, supported_device_targets, KoboTarget, StdDeviceProbe};
use kc_domain::{HostAccess, MountPoint};
use kc_host::{current_host_adapter, supported_host_adapters, FilesystemHost, StdHostFilesystem};

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();

    let result = match args.as_slice() {
        [] => foundation_report(),
        [command] if command == "foundation" || command == "report" => foundation_report(),
        [command, path] if command == "probe" => probe_path(Path::new(path)),
        _ => Err(kc_domain::DomainError::Validation(
            "invalid diagnostic command usage".to_string(),
        )),
    };

    if let Err(error) = result {
        print_usage();
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

fn foundation_report() -> Result<(), kc_domain::DomainError> {
    let current = current_host_adapter();
    println!("KOReader Companion foundation report");
    println!(
        "current host: {} ({:?})",
        current.descriptor.display_name, current.descriptor.kind
    );
    println!(
        "current host capabilities: {}",
        format_capabilities(&current.capabilities)
    );
    println!();
    println!("known host adapters:");
    for adapter in supported_host_adapters() {
        println!(
            "- {} [{}]: {}",
            adapter.descriptor.display_name,
            adapter.descriptor.id,
            format_capabilities(&adapter.capabilities)
        );
    }
    println!();
    println!("known device targets:");
    for target in supported_device_targets() {
        println!(
            "- {} [{} {:?}]: {}",
            target.descriptor.display_name,
            target.descriptor.id,
            target.descriptor.support_level,
            format_capabilities(&target.capabilities)
        );
    }

    Ok(())
}

fn probe_path(path: &Path) -> Result<(), kc_domain::DomainError> {
    let adapter = current_host_adapter();
    let host = FilesystemHost::with_mounts(
        adapter,
        StdHostFilesystem,
        vec![MountPoint {
            id: "manual-probe".to_string(),
            root: path.to_path_buf(),
            name: Some(path_display_name(path)),
            removable: false,
        }],
    );
    let validated = host.validate_manual_path(path)?;
    let kobo = KoboTarget::new(StdDeviceProbe);
    println!(
        "warning: manual probe does not verify that {} is a removable device mount",
        validated.path.display()
    );
    let assessments = assess_host_mounts(
        &FilesystemHost::with_mounts(
            current_host_adapter(),
            StdHostFilesystem,
            vec![MountPoint {
                id: "manual-probe".to_string(),
                root: validated.path,
                name: Some(path_display_name(path)),
                removable: false,
            }],
        ),
        &[&kobo],
    )?;

    for assessment in assessments {
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
        if assessment.readiness.ready {
            println!("install root: {}", assessment.install_root.display());
            println!("backup root: {}", assessment.backup_root.display());
        } else {
            println!(
                "install root: {} (device not ready; informational only)",
                assessment.install_root.display()
            );
            println!(
                "backup root: {} (device not ready; informational only)",
                assessment.backup_root.display()
            );
        }
    }

    Ok(())
}

fn format_capabilities(capabilities: &kc_domain::CapabilityProfile) -> String {
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

fn path_display_name(path: &Path) -> String {
    path.file_name()
        .map(|name| name.to_string_lossy().into_owned())
        .unwrap_or_else(|| PathBuf::from(path).display().to_string())
}

fn print_usage() {
    println!("Usage:");
    println!("  kc-diagnostic foundation");
    println!("  kc-diagnostic probe <device-root>");
}

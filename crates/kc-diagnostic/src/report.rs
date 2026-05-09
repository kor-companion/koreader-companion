use std::fs;
use std::path::{Component, Path, PathBuf};

use kc_device::{supported_device_targets, KoboTarget, StdDeviceProbe};
use kc_domain::{Address, DeviceTarget, HostAccess, HostOperationTarget, MountPoint};
use kc_host::{current_host_adapter, supported_host_adapters, FilesystemHost, StdHostFilesystem};

use crate::output::{
    format_capabilities, path_display_name, print_assessment, print_host_operation,
};

pub fn foundation_report() -> Result<(), kc_domain::DomainError> {
    for line in foundation_report_lines() {
        if line.is_empty() {
            println!();
        } else {
            println!("{line}");
        }
    }

    Ok(())
}

pub fn foundation_report_lines() -> Vec<String> {
    let current = current_host_adapter();
    let mut lines = vec![
        "KOReader Companion foundation report".to_string(),
        format!(
            "current host: {} ({:?})",
            current.descriptor.display_name, current.descriptor.kind
        ),
        format!(
            "current host capabilities: {}",
            format_capabilities(&current.capabilities)
        ),
        String::new(),
        "known host adapters:".to_string(),
    ];

    for adapter in supported_host_adapters() {
        lines.push(format!(
            "- {} [{}]: {}",
            adapter.descriptor.display_name,
            adapter.descriptor.id,
            format_capabilities(&adapter.capabilities)
        ));
    }

    lines.push(String::new());
    lines.push("known device targets:".to_string());

    for target in supported_device_targets() {
        lines.push(format!(
            "- {} [{} {:?}]: {}",
            target.descriptor.display_name,
            target.descriptor.id,
            target.descriptor.support_level,
            format_capabilities(&target.capabilities)
        ));
    }

    lines
}

pub fn probe_path(path: &Path) -> Result<(), kc_domain::DomainError> {
    let requested_root = normalize_manual_probe_root(path)?;
    let requested_metadata = fs::symlink_metadata(&requested_root).map_err(|error| {
        kc_domain::DomainError::Validation(format!(
            "failed to inspect manual probe path {}: {error}",
            requested_root.display()
        ))
    })?;
    if requested_metadata.file_type().is_symlink() {
        return Err(kc_domain::DomainError::Validation(format!(
            "manual probe path must not be a symlink: {}",
            requested_root.display()
        )));
    }
    let adapter = current_host_adapter();
    let mount = MountPoint {
        id: "manual-probe".to_string(),
        root: requested_root.clone(),
        name: Some(path_display_name(&requested_root)),
        removable: false,
    };
    let host = FilesystemHost::with_mounts(adapter, StdHostFilesystem, vec![mount.clone()]);
    let validated = host.validate_manual_path(&requested_root)?;
    if validated.path != requested_root {
        return Err(kc_domain::DomainError::Validation(format!(
            "validated manual probe path {} resolved outside requested root {}",
            validated.path.display(),
            requested_root.display()
        )));
    }
    let kobo = KoboTarget::new(StdDeviceProbe);
    let validated_mount = MountPoint {
        root: validated.path.clone(),
        ..mount
    };
    let validated_host = FilesystemHost::with_mounts(
        current_host_adapter(),
        StdHostFilesystem,
        vec![validated_mount.clone()],
    );

    println!(
        "warning: manual probe does not verify that {} is a removable device mount",
        validated.path.display()
    );

    let readiness = kobo.readiness(&validated_mount)?;
    let install_readiness =
        validated_host.sync_readiness(&HostOperationTarget::Mount(validated_mount.clone()))?;
    let eject_readiness =
        validated_host.eject_readiness(&HostOperationTarget::Mount(validated_mount.clone()))?;

    let (install_target, backup_target, install_root, backup_root) = if readiness.ready {
        (
            Some(kobo.install_target(&validated_mount)?),
            Some(kobo.backup_target(&validated_mount)?),
            Some(kobo.install_root(&validated_mount)?),
            Some(kobo.backup_root(&validated_mount)?),
        )
    } else {
        (None, None, None, None)
    };

    print_assessment(
        &validated_mount,
        kobo.descriptor(),
        &readiness,
        install_target.as_ref(),
        backup_target.as_ref(),
        install_root.as_deref(),
        backup_root.as_deref(),
    );
    print_host_operation("sync", &install_readiness);
    print_host_operation("eject", &eject_readiness);

    let metadata = validated_host.read_metadata(&Address::filesystem(
        validated_mount.root.join(".kobo/Kobo/Kobo eReader.conf"),
    ))?;
    println!(
        "config metadata: exists={} kind={:?} read_only={:?}",
        metadata.exists, metadata.kind, metadata.read_only
    );

    Ok(())
}

fn normalize_manual_probe_root(path: &Path) -> Result<PathBuf, kc_domain::DomainError> {
    let candidate = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .map_err(|error| {
                kc_domain::DomainError::Validation(format!(
                    "failed to resolve current directory for manual probe: {error}"
                ))
            })?
            .join(path)
    };

    let mut normalized = PathBuf::new();
    for component in candidate.components() {
        match component {
            Component::Prefix(prefix) => normalized.push(prefix.as_os_str()),
            Component::RootDir => normalized.push(component.as_os_str()),
            Component::CurDir => {}
            Component::Normal(part) => normalized.push(part),
            Component::ParentDir => {
                return Err(kc_domain::DomainError::Validation(format!(
                    "manual probe path must not contain parent traversal: {}",
                    path.display()
                )));
            }
        }
    }

    Ok(normalized)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foundation_report_mentions_supported_targets() {
        let lines = foundation_report_lines();

        assert!(lines
            .iter()
            .any(|line| line == "KOReader Companion foundation report"));
        assert!(lines
            .iter()
            .any(|line| line.contains("known host adapters:")));
        assert!(lines
            .iter()
            .any(|line| line.contains("known device targets:")));
        assert!(lines.iter().any(|line| {
            line.contains("Kobo USB mass storage target") && line.contains("Supported")
        }));
    }

    #[test]
    fn manual_probe_root_normalization_preserves_absolute_roots() {
        let absolute = if cfg!(windows) {
            Path::new(r"C:\probe\root")
        } else {
            Path::new("/probe/root")
        };

        let normalized = normalize_manual_probe_root(absolute).unwrap();

        assert!(normalized.is_absolute());
        assert_eq!(normalized, absolute);
    }
}

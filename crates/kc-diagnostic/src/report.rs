use std::path::Path;

use kc_device::{assess_host_mounts, supported_device_targets, KoboTarget, StdDeviceProbe};
use kc_domain::{Address, HostAccess, HostOperationTarget, MountPoint};
use kc_host::{current_host_adapter, supported_host_adapters, FilesystemHost, StdHostFilesystem};

use crate::output::{
    format_capabilities, path_display_name, print_assessment, print_host_operation,
};

pub fn foundation_report() -> Result<(), kc_domain::DomainError> {
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

pub fn probe_path(path: &Path) -> Result<(), kc_domain::DomainError> {
    let adapter = current_host_adapter();
    let mount = MountPoint {
        id: "manual-probe".to_string(),
        root: path.to_path_buf(),
        name: Some(path_display_name(path)),
        removable: false,
    };
    let host = FilesystemHost::with_mounts(adapter, StdHostFilesystem, vec![mount.clone()]);
    let validated = host.validate_manual_path(path)?;
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

    let assessments = assess_host_mounts(&validated_host, &[&kobo])?;
    for assessment in assessments {
        let install_readiness =
            validated_host.sync_readiness(&HostOperationTarget::Mount(assessment.mount.clone()))?;
        let eject_readiness = validated_host
            .eject_readiness(&HostOperationTarget::Mount(assessment.mount.clone()))?;

        print_assessment(assessment);
        print_host_operation("sync", &install_readiness);
        print_host_operation("eject", &eject_readiness);

        let metadata = validated_host.read_metadata(&Address::filesystem(
            validated_mount.root.join(".kobo/Kobo/Kobo eReader.conf"),
        ))?;
        println!(
            "config metadata: exists={} kind={:?} read_only={:?}",
            metadata.exists, metadata.kind, metadata.read_only
        );
    }

    Ok(())
}

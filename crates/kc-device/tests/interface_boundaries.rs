use std::path::PathBuf;

use kc_device::{assess_host_mounts, InMemoryDeviceProbe, KoboTarget, StaticDeviceTarget};
use kc_domain::{
    Address, CapabilityProfile, DeviceDescriptor, DeviceKind, MountPoint, ReadinessReport,
    SupportLevel, TransportKind,
};
use kc_host::{FilesystemHost, HostAdapterDescriptor, InMemoryHostFilesystem};

#[test]
fn host_and_device_adapters_are_exercised_through_interfaces() {
    let host = FilesystemHost::with_mounts(
        HostAdapterDescriptor::linux(),
        InMemoryHostFilesystem::new([
            PathBuf::from("/mnt/kobo"),
            PathBuf::from("/mnt/future-reader"),
        ]),
        vec![
            MountPoint {
                id: "kobo".to_string(),
                root: PathBuf::from("/mnt/kobo"),
                name: Some("KOBOeReader".to_string()),
                removable: true,
            },
            MountPoint {
                id: "future".to_string(),
                root: PathBuf::from("/mnt/future-reader"),
                name: Some("Future Reader".to_string()),
                removable: true,
            },
        ],
    );

    let kobo = KoboTarget::new(InMemoryDeviceProbe::new(
        [PathBuf::from("/mnt/kobo/.kobo")],
        [PathBuf::from("/mnt/kobo/.kobo/Kobo/Kobo eReader.conf")],
    ));
    let future = StaticDeviceTarget::new(
        DeviceDescriptor {
            id: "future-target".to_string(),
            kind: DeviceKind::PocketBook,
            display_name: "Future target seam".to_string(),
            support_level: SupportLevel::Unsupported,
        },
        CapabilityProfile::default(),
        ReadinessReport::blocked(vec!["target not implemented in this grouping".to_string()]),
        ".adds/app",
        "system",
    )
    .unwrap();

    let assessments = assess_host_mounts(&host, &[&kobo, &future]).unwrap();
    assert_eq!(assessments.len(), 4);

    let kobo_assessment = assessments
        .iter()
        .find(|assessment| {
            assessment.mount.id == "kobo" && assessment.descriptor.id == "kobo-usb-mass-storage"
        })
        .unwrap();
    assert!(kobo_assessment.readiness.ready);

    let future_assessment = assessments
        .iter()
        .find(|assessment| {
            assessment.mount.id == "future" && assessment.descriptor.id == "future-target"
        })
        .unwrap();
    assert!(!future_assessment.readiness.ready);
    assert_eq!(future_assessment.install_root, None);
    assert_eq!(future_assessment.backup_root, None);
    assert_eq!(future_assessment.install_target, None);
    assert_eq!(future_assessment.backup_target, None);

    let kobo_assessment = assessments
        .iter()
        .find(|assessment| {
            assessment.mount.id == "kobo" && assessment.descriptor.id == "kobo-usb-mass-storage"
        })
        .unwrap();
    assert_eq!(
        kobo_assessment.install_root,
        Some(PathBuf::from("/mnt/kobo/.adds/koreader"))
    );
    assert!(matches!(
        kobo_assessment.install_target,
        Some(Address::ScopedPath {
            transport: TransportKind::UsbMassStorage,
            ..
        })
    ));
}

#[test]
fn symlinked_kobo_markers_stay_blocked_through_device_interfaces() {
    let host = FilesystemHost::with_mounts(
        HostAdapterDescriptor::linux(),
        InMemoryHostFilesystem::new([PathBuf::from("/mnt/kobo")]),
        vec![MountPoint {
            id: "kobo".to_string(),
            root: PathBuf::from("/mnt/kobo"),
            name: Some("KOBOeReader".to_string()),
            removable: true,
        }],
    );

    let kobo = KoboTarget::new(InMemoryDeviceProbe::new([], []).with_symlinks([
        PathBuf::from("/mnt/kobo/.kobo"),
        PathBuf::from("/mnt/kobo/.kobo/Kobo/Kobo eReader.conf"),
    ]));

    let assessments = assess_host_mounts(&host, &[&kobo]).unwrap();
    assert_eq!(assessments.len(), 1);

    let assessment = &assessments[0];
    assert_eq!(assessment.descriptor.id, "kobo-usb-mass-storage");
    assert!(!assessment.readiness.ready);
    assert_eq!(
        assessment.readiness.blockers,
        vec![
            "missing required .kobo directory".to_string(),
            "missing Kobo config .kobo/Kobo/Kobo eReader.conf".to_string(),
        ]
    );
    assert_eq!(assessment.install_target, None);
    assert_eq!(assessment.backup_target, None);
    assert_eq!(assessment.install_root, None);
    assert_eq!(assessment.backup_root, None);
}

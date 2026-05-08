use kc_domain::{Capability, CapabilityProfile, HostDescriptor, HostKind};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HostAdapterDescriptor {
    pub descriptor: HostDescriptor,
    pub capabilities: CapabilityProfile,
}

impl HostAdapterDescriptor {
    pub fn linux() -> Self {
        Self {
            descriptor: HostDescriptor {
                id: "linux-desktop".to_string(),
                kind: HostKind::Linux,
                display_name: "Linux desktop host".to_string(),
            },
            capabilities: CapabilityProfile::new([
                Capability::SupportsDirectFilesystemAccess,
                Capability::SupportsSafeEject,
            ]),
        }
    }

    pub fn macos() -> Self {
        Self {
            descriptor: HostDescriptor {
                id: "macos-desktop".to_string(),
                kind: HostKind::MacOs,
                display_name: "macOS desktop host".to_string(),
            },
            capabilities: CapabilityProfile::new([Capability::SupportsDirectFilesystemAccess]),
        }
    }

    pub fn windows() -> Self {
        Self {
            descriptor: HostDescriptor {
                id: "windows-desktop".to_string(),
                kind: HostKind::Windows,
                display_name: "Windows desktop host".to_string(),
            },
            capabilities: CapabilityProfile::new([Capability::SupportsDirectFilesystemAccess]),
        }
    }

    pub fn other(id: impl Into<String>, display_name: impl Into<String>) -> Self {
        let id = id.into();
        Self {
            descriptor: HostDescriptor {
                kind: HostKind::Other(id.clone()),
                id,
                display_name: display_name.into(),
            },
            capabilities: CapabilityProfile::new([Capability::SupportsDirectFilesystemAccess]),
        }
    }
}

pub fn supported_host_adapters() -> Vec<HostAdapterDescriptor> {
    vec![
        HostAdapterDescriptor::linux(),
        HostAdapterDescriptor::macos(),
        HostAdapterDescriptor::windows(),
    ]
}

pub fn current_host_adapter() -> HostAdapterDescriptor {
    match std::env::consts::OS {
        "linux" => HostAdapterDescriptor::linux(),
        "macos" => HostAdapterDescriptor::macos(),
        "windows" => HostAdapterDescriptor::windows(),
        other => HostAdapterDescriptor::other(other, format!("{other} host")),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn desktop_host_descriptors_are_exposed() {
        let adapters = supported_host_adapters();
        assert_eq!(adapters.len(), 3);
        assert_eq!(adapters[0].descriptor.kind, HostKind::Linux);
        assert_eq!(adapters[1].descriptor.kind, HostKind::MacOs);
        assert_eq!(adapters[2].descriptor.kind, HostKind::Windows);
    }
}

use std::path::{Component, Path, PathBuf};

use kc_domain::{Address, ContainmentPolicy, DomainError, MountPoint, TransportKind};

pub fn contained_mount_address(
    mount: &MountPoint,
    relative: &Path,
) -> Result<Address, DomainError> {
    let relative = normalize_relative_root(relative.to_path_buf())?;
    ContainmentPolicy::new(&mount.root)?.contain(&relative)?;
    Address::scoped(TransportKind::UsbMassStorage, mount.id.clone(), relative)
}

pub fn normalize_relative_root(path: PathBuf) -> Result<PathBuf, DomainError> {
    if path.as_os_str().is_empty() || path.is_absolute() {
        return Err(DomainError::Validation(format!(
            "device target root must be a non-empty relative path: {}",
            path.display()
        )));
    }

    let mut normalized = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::Normal(part) => normalized.push(part),
            Component::ParentDir | Component::Prefix(_) | Component::RootDir => {
                return Err(DomainError::Validation(format!(
                    "device target root must stay within the mount boundary: {}",
                    path.display()
                )));
            }
        }
    }

    if normalized.as_os_str().is_empty() {
        return Err(DomainError::Validation(
            "device target root must not resolve to an empty path".to_string(),
        ));
    }

    Ok(normalized)
}

pub fn contained_mount_path(mount: &MountPoint, relative: &Path) -> Result<PathBuf, DomainError> {
    Ok(ContainmentPolicy::new(&mount.root)?
        .contain(relative)?
        .full_path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn contained_mount_addresses_use_scoped_usb_shape() {
        let mount = MountPoint {
            id: "kobo-1".to_string(),
            root: PathBuf::from("/mnt/kobo"),
            name: Some("KOBOeReader".to_string()),
            removable: true,
        };

        let address = contained_mount_address(&mount, Path::new(".adds/koreader")).unwrap();
        assert!(matches!(
            address,
            Address::ScopedPath {
                transport: TransportKind::UsbMassStorage,
                scope,
                relative_path,
            } if scope == "kobo-1" && relative_path == PathBuf::from(".adds/koreader")
        ));
    }

    #[test]
    fn contained_mount_addresses_reject_empty_mount_scope() {
        let mount = MountPoint {
            id: "".to_string(),
            root: PathBuf::from("/mnt/kobo"),
            name: Some("KOBOeReader".to_string()),
            removable: true,
        };

        let error = contained_mount_address(&mount, Path::new(".adds/koreader")).unwrap_err();
        assert!(matches!(error, DomainError::Validation(_)));
    }
}

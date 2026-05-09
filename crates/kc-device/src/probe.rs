use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

pub trait DeviceRootProbe {
    fn is_dir(&self, root: &Path, relative: &Path) -> bool;
    fn is_file(&self, root: &Path, relative: &Path) -> bool;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StdDeviceProbe;

impl DeviceRootProbe for StdDeviceProbe {
    fn is_dir(&self, root: &Path, relative: &Path) -> bool {
        fs::symlink_metadata(root.join(relative))
            .map(|metadata| metadata.file_type().is_dir())
            .unwrap_or(false)
    }

    fn is_file(&self, root: &Path, relative: &Path) -> bool {
        fs::symlink_metadata(root.join(relative))
            .map(|metadata| metadata.file_type().is_file())
            .unwrap_or(false)
    }
}

#[derive(Debug, Clone, Default)]
pub struct InMemoryDeviceProbe {
    directories: BTreeSet<PathBuf>,
    files: BTreeSet<PathBuf>,
    symlinks: BTreeSet<PathBuf>,
}

impl InMemoryDeviceProbe {
    pub fn new(
        directories: impl IntoIterator<Item = PathBuf>,
        files: impl IntoIterator<Item = PathBuf>,
    ) -> Self {
        Self {
            directories: directories.into_iter().collect(),
            files: files.into_iter().collect(),
            symlinks: BTreeSet::new(),
        }
    }

    pub fn with_symlinks(mut self, paths: impl IntoIterator<Item = PathBuf>) -> Self {
        self.symlinks = paths.into_iter().collect();
        self
    }
}

impl DeviceRootProbe for InMemoryDeviceProbe {
    fn is_dir(&self, root: &Path, relative: &Path) -> bool {
        let path = root.join(relative);
        self.directories.contains(&path) && !self.symlinks.contains(&path)
    }

    fn is_file(&self, root: &Path, relative: &Path) -> bool {
        let path = root.join(relative);
        self.files.contains(&path) && !self.symlinks.contains(&path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn symlink_like_entries_do_not_pass_readiness_checks() {
        let probe = InMemoryDeviceProbe::new([], []).with_symlinks([
            PathBuf::from("/mnt/kobo/.kobo"),
            PathBuf::from("/mnt/kobo/.kobo/Kobo/Kobo eReader.conf"),
        ]);

        assert!(!probe.is_dir(Path::new("/mnt/kobo"), Path::new(".kobo")));
        assert!(!probe.is_file(
            Path::new("/mnt/kobo"),
            Path::new(".kobo/Kobo/Kobo eReader.conf")
        ));
    }
}

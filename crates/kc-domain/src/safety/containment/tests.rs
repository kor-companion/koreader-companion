use std::fs;
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

use super::{ContainmentPolicy, SafetyViolation};

#[cfg(unix)]
use std::os::unix::fs::symlink;

#[test]
fn containment_policy_rejects_escape_paths() {
    let policy = ContainmentPolicy::new("/mnt/kobo").unwrap();

    let contained = policy
        .contain(Path::new(".adds/../.adds/koreader"))
        .unwrap();
    assert_eq!(
        contained.full_path,
        PathBuf::from("/mnt/kobo/.adds/koreader")
    );
    assert_eq!(contained.relative_path, PathBuf::from(".adds/koreader"));

    let error = policy.contain(Path::new("../etc/passwd")).unwrap_err();
    assert_eq!(
        error,
        SafetyViolation::PathOutsideRoot {
            root: PathBuf::from("/mnt/kobo"),
            candidate: PathBuf::from("/mnt/etc/passwd"),
        }
    );
}

#[cfg(unix)]
#[test]
fn containment_policy_rejects_symlink_components() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("kc-domain-{unique}"));
    let outside = std::env::temp_dir().join(format!("kc-domain-outside-{unique}"));

    fs::create_dir_all(&root).unwrap();
    fs::create_dir_all(&outside).unwrap();
    symlink(&outside, root.join("escape")).unwrap();

    let policy = ContainmentPolicy::new(&root).unwrap();
    let error = policy.contain(Path::new("escape/file.txt")).unwrap_err();
    assert_eq!(
        error,
        SafetyViolation::SymlinkComponent(root.join("escape"))
    );

    fs::remove_file(root.join("escape")).unwrap();
    fs::remove_dir_all(&root).unwrap();
    fs::remove_dir_all(&outside).unwrap();
}

#[cfg(unix)]
#[test]
fn containment_policy_rejects_root_symlink_created_after_init() {
    let unique = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = std::env::temp_dir().join(format!("kc-domain-late-root-{unique}"));
    let outside = std::env::temp_dir().join(format!("kc-domain-late-outside-{unique}"));

    let policy = ContainmentPolicy::new(&root).unwrap();
    fs::create_dir_all(&outside).unwrap();
    symlink(&outside, &root).unwrap();

    let error = policy.contain(Path::new("escape/file.txt")).unwrap_err();
    assert_eq!(error, SafetyViolation::SymlinkComponent(root.clone()));

    fs::remove_file(&root).unwrap();
    fs::remove_dir_all(&outside).unwrap();
}

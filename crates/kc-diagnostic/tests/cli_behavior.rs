use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

struct TestDir {
    path: PathBuf,
}

impl TestDir {
    fn new(name: &str) -> Self {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!(
            "kc-diagnostic-{name}-{}-{unique}",
            std::process::id()
        ));
        fs::create_dir_all(&path).unwrap();
        Self { path }
    }

    fn path(&self) -> &Path {
        &self.path
    }
}

impl Drop for TestDir {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.path);
    }
}

#[test]
fn foundation_command_reports_known_foundation_surfaces() {
    let output = run_diagnostic(&["foundation"]);

    assert!(output.status.success());
    let stdout = stdout_string(&output);
    assert!(stdout.contains("KOReader Companion foundation report"));
    assert!(stdout.contains("known host adapters:"));
    assert!(stdout.contains("known device targets:"));
    assert!(stdout.contains("Kobo USB mass storage target"));
}

#[test]
fn valid_fake_kobo_probe_reports_supported_and_ready_state_separately() {
    let temp = TestDir::new("valid-kobo");
    let root = temp.path().join("KOBOeReader");

    fs::create_dir_all(root.join(".kobo/Kobo")).unwrap();
    fs::write(
        root.join(".kobo/Kobo/Kobo eReader.conf"),
        b"[ApplicationPreferences]\n",
    )
    .unwrap();

    let output = run_probe(&root);

    assert!(output.status.success());
    let stdout = stdout_string(&output);
    assert!(stdout.contains("target: Kobo USB mass storage target"));
    assert!(stdout.contains("support level: Supported"));
    assert!(stdout.contains("current readiness: ready"));
    assert!(stdout.contains("blockers: none"));
    assert!(stdout.contains("sync automation readiness: blocked"));
    assert!(stdout.contains("eject automation readiness: blocked"));
    assert!(stdout.contains("config metadata: exists=true kind=Some(File)"));
}

#[test]
fn non_kobo_probe_reports_supported_but_currently_blocked() {
    let temp = TestDir::new("non-kobo");
    let root = temp.path().join("random-usb");

    fs::create_dir_all(root.join("Documents")).unwrap();
    fs::write(root.join("Documents/notes.txt"), b"not a kobo").unwrap();

    let output = run_probe(&root);

    assert!(output.status.success());
    let stdout = stdout_string(&output);
    assert!(stdout.contains("target: Kobo USB mass storage target"));
    assert!(stdout.contains("support level: Supported"));
    assert!(stdout.contains("current readiness: blocked"));
    assert!(stdout.contains("- missing required .kobo directory"));
    assert!(stdout.contains("- missing Kobo config .kobo/Kobo/Kobo eReader.conf"));
    assert!(stdout.contains("config metadata: exists=false kind=None read_only=None"));
}

#[test]
fn empty_probe_reports_blocked_readiness() {
    let temp = TestDir::new("empty-probe");
    let root = temp.path().join("empty-root");

    fs::create_dir_all(&root).unwrap();

    let output = run_probe(&root);

    assert!(output.status.success());
    let stdout = stdout_string(&output);
    assert!(stdout.contains("current readiness: blocked"));
    assert!(stdout.contains("- missing required .kobo directory"));
    assert!(stdout.contains("- missing Kobo config .kobo/Kobo/Kobo eReader.conf"));
}

#[cfg(unix)]
#[test]
fn symlinked_kobo_markers_do_not_pass_probe_readiness() {
    use std::os::unix::fs::symlink;

    let temp = TestDir::new("symlink-probe");
    let root = temp.path().join("probe-root");
    let linked_kobo = temp.path().join("linked-kobo");
    let linked_config_target = temp.path().join("linked-config-target.conf");

    fs::create_dir_all(&root).unwrap();
    fs::create_dir_all(linked_kobo.join("Kobo")).unwrap();
    fs::write(&linked_config_target, b"[ApplicationPreferences]\n").unwrap();
    symlink(&linked_kobo, root.join(".kobo")).unwrap();
    symlink(
        &linked_config_target,
        linked_kobo.join("Kobo/Kobo eReader.conf"),
    )
    .unwrap();

    let output = run_probe(&root);

    assert!(output.status.success());
    let stdout = stdout_string(&output);
    assert!(stdout.contains("current readiness: blocked"));
    assert!(stdout.contains("- missing required .kobo directory"));
    assert!(stdout.contains("- missing Kobo config .kobo/Kobo/Kobo eReader.conf"));
    assert!(stdout.contains("config metadata: exists=true kind=Some(Symlink)"));
}

#[cfg(unix)]
#[test]
fn symlinked_probe_root_is_rejected() {
    use std::os::unix::fs::symlink;

    let temp = TestDir::new("symlink-root");
    let real_root = temp.path().join("real-root");
    let link_root = temp.path().join("link-root");

    fs::create_dir_all(&real_root).unwrap();
    symlink(&real_root, &link_root).unwrap();

    let output = run_probe(&link_root);

    assert!(!output.status.success());
    assert!(stderr_string(&output).contains("manual probe path must not be a symlink"));
}

#[test]
fn probe_rejects_parent_traversal_paths() {
    let temp = TestDir::new("parent-traversal");
    let outer = temp.path().join("outer");
    let nested = outer.join("nested");

    fs::create_dir_all(&nested).unwrap();
    let traversal = nested.join("..").join("..");

    let output = run_probe(&traversal);

    assert!(!output.status.success());
    assert!(stderr_string(&output).contains("manual probe path must not contain parent traversal"));
}

#[test]
fn invalid_usage_prints_usage_and_exits_non_zero() {
    let output = run_diagnostic(&["probe"]);

    assert!(!output.status.success());
    assert!(stdout_string(&output).contains("Usage:"));
    assert!(stderr_string(&output).contains("invalid diagnostic command usage"));
}

fn run_probe(path: &Path) -> Output {
    run_diagnostic(&["probe", path.to_str().unwrap()])
}

fn run_diagnostic(args: &[&str]) -> Output {
    Command::new(env!("CARGO_BIN_EXE_kc-diagnostic"))
        .args(args)
        .output()
        .unwrap()
}

fn stdout_string(output: &Output) -> String {
    String::from_utf8(output.stdout.clone()).unwrap()
}

fn stderr_string(output: &Output) -> String {
    String::from_utf8(output.stderr.clone()).unwrap()
}

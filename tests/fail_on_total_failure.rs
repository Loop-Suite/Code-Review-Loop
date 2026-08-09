//! Real subprocess E2E test for a gap found by external review and reproduced against the
//! actual compiled binary: `--fail-on` only ever checked the verdict *string*, and a run where
//! every lens fails (no defect-finding coverage at all -- e.g. the configured backend is
//! unreachable) computes the exact same "APPROVE" verdict a genuinely clean diff would, since
//! zero findings looks the same either way to `quantify::verdict()`. Before the fix, this meant
//! a completely broken review (wrong endpoint, provider outage, whatever) could still exit 0
//! under `--fail-on request-changes`, looking like a clean pass to any CI job gating on it.
//!
//! Unit-level coverage already exists in `src/pipeline/review.rs` for the underlying
//! `completeness == Failed` computation, but that alone wouldn't have caught this bug -- it was
//! specifically in `main.rs`'s wiring between `run_review`'s outcome and `std::process::exit`,
//! which only a real subprocess run actually exercises.

use std::io::Write;
use std::process::Command;

fn write_temp(name: &str, contents: &str) -> std::path::PathBuf {
    let dir = std::env::temp_dir().join("codereview-loop-fail-on-total-failure-test");
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(name);
    let mut f = std::fs::File::create(&path).unwrap();
    f.write_all(contents.as_bytes()).unwrap();
    path
}

#[test]
fn review_exits_non_zero_under_fail_on_when_every_lens_fails() {
    let spec_path = write_temp(
        "spec.toml",
        r#"
name = "e2e test spec"
labels = ["possible bug"]

[[lenses]]
id = "test_lens"
title = "Test Lens"
guide = "test"
always = true
"#,
    );
    let diff_path = write_temp(
        "diff.patch",
        "diff --git a/src/example.rs b/src/example.rs\n\
         --- a/src/example.rs\n\
         +++ b/src/example.rs\n\
         @@ -1,1 +1,1 @@\n\
         -old line\n\
         +new line\n",
    );
    let out_dir = std::env::temp_dir()
        .join("codereview-loop-fail-on-total-failure-test")
        .join("out");
    let _ = std::fs::remove_dir_all(&out_dir);

    let bin = env!("CARGO_BIN_EXE_codereview");
    let output = Command::new(bin)
        .args([
            "review",
            "--spec",
            spec_path.to_str().unwrap(),
            "--diff",
            diff_path.to_str().unwrap(),
            "--backend",
            "custom",
            // Port 9 (historically "discard") refused-connection on loopback -- no network call
            // ever actually leaves the machine, and the failure is immediate, not a hang.
            "--base-url",
            "http://127.0.0.1:9/v1/chat/completions",
            "--model",
            "irrelevant",
            "--out",
            out_dir.to_str().unwrap(),
            "--max-rounds",
            "1",
            "--fail-on",
            "request-changes",
        ])
        .output()
        .expect("failed to run the codereview binary");

    assert!(
        !output.status.success(),
        "a run where every lens failed must exit non-zero under --fail-on, regardless of the \
         verdict string it happened to compute -- stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

#[test]
fn review_still_exits_zero_under_fail_on_never_when_every_lens_fails() {
    // The escape hatch: --fail-on never must still mean "advisory only, exit 0 no matter what,"
    // even for a completely failed run -- this isn't about making every failure fatal
    // unconditionally, only about --fail-on's own opt-in gate actually gating on the right thing.
    let spec_path = write_temp(
        "spec2.toml",
        r#"
name = "e2e test spec"
labels = ["possible bug"]

[[lenses]]
id = "test_lens"
title = "Test Lens"
guide = "test"
always = true
"#,
    );
    let diff_path = write_temp(
        "diff2.patch",
        "diff --git a/src/example.rs b/src/example.rs\n\
         --- a/src/example.rs\n\
         +++ b/src/example.rs\n\
         @@ -1,1 +1,1 @@\n\
         -old line\n\
         +new line\n",
    );
    let out_dir = std::env::temp_dir()
        .join("codereview-loop-fail-on-total-failure-test")
        .join("out2");
    let _ = std::fs::remove_dir_all(&out_dir);

    let bin = env!("CARGO_BIN_EXE_codereview");
    let output = Command::new(bin)
        .args([
            "review",
            "--spec",
            spec_path.to_str().unwrap(),
            "--diff",
            diff_path.to_str().unwrap(),
            "--backend",
            "custom",
            "--base-url",
            "http://127.0.0.1:9/v1/chat/completions",
            "--model",
            "irrelevant",
            "--out",
            out_dir.to_str().unwrap(),
            "--max-rounds",
            "1",
            "--fail-on",
            "never",
        ])
        .output()
        .expect("failed to run the codereview binary");

    assert!(
        output.status.success(),
        "--fail-on never must still exit 0 even on a completely failed run -- stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

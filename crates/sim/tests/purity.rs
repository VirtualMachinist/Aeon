//! QA gate S1: the sim is a pure integer function of its inputs.
//!
//! No floats, no clock, no filesystem, no randomness, no renderer or input
//! crates. Checked mechanically over the crate's own sources and manifest so
//! the gate cannot rot silently.

use std::fs;
use std::path::{Path, PathBuf};

fn sim_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn rust_sources(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).expect("read sim src") {
        let path = entry.expect("dir entry").path();
        if path.is_dir() {
            rust_sources(&path, out);
        } else if path.extension().is_some_and(|e| e == "rs") {
            out.push(path);
        }
    }
}

#[test]
fn sim_has_no_dependencies() {
    let manifest = fs::read_to_string(sim_root().join("Cargo.toml")).unwrap();
    let deps = manifest
        .split("[dependencies]")
        .nth(1)
        .unwrap_or("")
        .lines()
        .take_while(|l| !l.starts_with('['))
        .filter(|l| !l.trim().is_empty() && !l.trim_start().starts_with('#'))
        .count();
    assert_eq!(deps, 0, "aeon-sim must stay dependency-free: {manifest}");
    for banned in ["macroquad", "gilrs", "kira", "bevy", "rand", "serde"] {
        assert!(!manifest.contains(banned), "aeon-sim manifest mentions {banned}");
    }
}

#[test]
fn sim_sources_have_no_floats_clock_fs_or_randomness() {
    let mut files = Vec::new();
    rust_sources(&sim_root().join("src"), &mut files);
    assert!(files.len() > 5, "expected the sim sources, found {files:?}");
    let banned = [
        "f32",
        "f64",
        "std::fs",
        "std::time",
        "Instant",
        "SystemTime",
        "rand",
        "macroquad",
        "gilrs",
        "std::io",
    ];
    for file in files {
        let text = fs::read_to_string(&file).unwrap();
        for (n, line) in text.lines().enumerate() {
            // Doc comments may describe what is *not* allowed.
            let code = line.trim_start();
            if code.starts_with("//") {
                continue;
            }
            for b in banned {
                assert!(
                    !code.contains(b),
                    "{}:{} uses `{b}`: {line}",
                    file.display(),
                    n + 1
                );
            }
        }
    }
}

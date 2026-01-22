use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=proto");

    if !Path::new("proto/model/ciphertext.proto").exists() {
        fetch_protos();
    }

    let protoc = std::env::var("PROTOC")
        .ok()
        .map(|p| Path::new(&p).to_path_buf())
        .filter(|p| p.exists())
        .or_else(|| {
            Command::new("which")
                .arg("protoc")
                .output()
                .ok()
                .filter(|o| o.status.success())
                .and_then(|o| {
                    String::from_utf8(o.stdout).ok().map(|s| s.trim().to_string())
                })
                .map(|s| Path::new(&s).to_path_buf())
        })
        .or_else(|| protoc_bin_vendored::protoc_bin_path().ok())
        .expect("Could not find protoc. Install protoc or ensure protoc-bin-vendored works.");

    let mut config = prost_build::Config::new();
    config.protoc_executable(protoc.to_str().expect("Invalid protoc path"));
    config
        .compile_protos(&["proto/model/ciphertext.proto"][..], &["proto/"][..])
        .unwrap();
}

fn fetch_protos() {
    let temp_dir = std::env::temp_dir().join(format!("ctes-proto-{}", std::process::id()));
    let model_dir = Path::new("proto/model");

    // Clone repo
    let status = Command::new("git")
        .args([
            "clone",
            "--depth=1",
            "--branch=main",
            "https://github.com/CiphertextEcosystem/CiphertextEcosystemProtobuf.git",
            temp_dir.to_str().unwrap(),
        ])
        .status()
        .expect("Failed to execute git");

    if !status.success() {
        panic!("Failed to clone protobuf repository");
    }

    std::fs::create_dir_all(model_dir).unwrap();
    copy_dir_all(&temp_dir.join("model"), model_dir).unwrap();
    std::fs::remove_dir_all(&temp_dir).ok();
}

fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(dst)?;
    for entry in std::fs::read_dir(src)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            copy_dir_all(&path, &dst.join(entry.file_name()))?;
        } else {
            std::fs::copy(&path, &dst.join(entry.file_name()))?;
        }
    }
    Ok(())
}

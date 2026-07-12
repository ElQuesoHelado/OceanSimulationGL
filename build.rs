// build.rs
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=assets");

    let out_dir = env::var("OUT_DIR").unwrap();
    let target_dir = PathBuf::from(&out_dir)
        .ancestors()
        .nth(3)
        .unwrap()
        .to_path_buf();

    let dest = target_dir.join("assets");
    let src = Path::new("assets");

    if src.exists() {
        copy_dir_all(src, &dest).expect("No se pudo copiar assets");
    }
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        let dest_path = dst.as_ref().join(entry.file_name());
        if ty.is_dir() {
            copy_dir_all(entry.path(), dest_path)?;
        } else {
            fs::copy(entry.path(), dest_path)?;
        }
    }
    Ok(())
}

use std::{fs, path::PathBuf};

fn main() {
    let dest_path = PathBuf::from("src").join("generated_at_build.rs");

    let build_id = nanoid::nanoid!(4).replace(['-'], "");

    fs::write(
        dest_path,
        format!("pub const BUILD_ID: &str = \"{}\";", build_id),
    )
    .unwrap();
}

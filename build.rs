use std::{
    env,
    path::PathBuf,
};

use fs_extra::dir;

fn get_output_path() -> PathBuf {
    confy::get_configuration_file_path("grande-sonnerie", None)
    .unwrap_or_default()
    .parent()
    .expect("Why is empty")
    .to_path_buf()
}

fn main() {
    let default_theme = "coucou";

    let src = env::current_dir().unwrap().join(default_theme);
    let target = get_output_path();

    dir::create_all(&target, false).unwrap_or_default();
    dir::copy(&src, &target, &dir::CopyOptions::new()).unwrap_or_default();
}

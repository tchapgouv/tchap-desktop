#[cfg(target_os = "macos")]
mod macos_build;    


fn main() {
    tauri_build::build();

    #[cfg(target_os = "macos")]
    macos_build::setup_sqlcipher();
}
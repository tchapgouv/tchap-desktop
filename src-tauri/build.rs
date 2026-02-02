use std::env;
use std::path::PathBuf;

fn main() {
    tauri_build::build();

    #[cfg(target_os = "macos")]
    {
        setup_sqlcipher_macos();
    }
}

#[cfg(target_os = "macos")]
fn setup_sqlcipher_macos() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let sqlcipher_dir = PathBuf::from(&manifest_dir)
        .join("../vendor/sqlcipher-macos-arm64");
    let openssl_dir = PathBuf::from(&manifest_dir)
        .join("../vendor/openssl-macos-arm64");
    
    if !sqlcipher_dir.exists() {
        panic!("SQLCipher not found! Run: ./scripts/build-sqlcipher-macos-arm64.sh");
    }
    
    // Tell Cargo where to find the static libs
    println!("cargo:rustc-link-search=native={}/lib", sqlcipher_dir.display());
    println!("cargo:rustc-link-search=native={}/lib", openssl_dir.display());
    
    // Link statically
    println!("cargo:rustc-link-lib=static=sqlcipher");
    println!("cargo:rustc-link-lib=static=crypto");
    
    // Framework necessary for macOS
    println!("cargo:rustc-link-lib=framework=Security");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    
    println!("cargo:rerun-if-changed=../vendor");
}

use std::env;
use std::path::PathBuf;

pub fn setup_sqlcipher() {
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
    
    println!("cargo:rerun-if-changed=../vendor");
}

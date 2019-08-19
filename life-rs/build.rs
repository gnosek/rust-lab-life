#[cfg(feature = "ffi_import")]
fn link_to_cpp_lib() {
    println!("cargo:rustc-link-search={}", ::std::env::var("CPP_DIR").unwrap());
    println!("cargo:rustc-link-lib=static=board");

    #[cfg(target_os = "macos")]
    println!("cargo:rustc-link-lib=c++");

    #[cfg(target_os = "linux")]
    println!("cargo:rustc-link-lib=stdc++");
}

fn main() {
    #[cfg(feature = "ffi_import")]
    link_to_cpp_lib();
}

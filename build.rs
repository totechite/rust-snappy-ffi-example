#[cfg(target_os = "windows")]
fn main() {
//  Reference to https://doc.rust-lang.org/cargo/reference/build-scripts.html
//  Please rewrite <path_to_vcpkg> to your setted path.

//  static library(*.lib)
    println!("cargo:rustc-link-search=native=<path_to_vcpkg>/installed/x64-windows/lib/");
//  dynamic library(*.dll)
    println!("cargo:rustc-env=PATH=<path_to_vcpkg>/installed/x64-windows/bin/");
}

#[cfg(target_os = "linux")]
fn main() {
//  Do nothing
}


//  This part is for UNIX like MacOS and others.
//  If it need to scripting and you can give it a favor, please complete me.
/*
#[cfg(target_os = "unix")]
fn main() {
    Maybe something is written.
}
*/
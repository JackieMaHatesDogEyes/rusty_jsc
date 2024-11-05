#[cfg(target_os = "macos")]
fn main() {
    println!("cargo:rustc-link-lib=framework=JavaScriptCore");
}

#[cfg(target_os = "linux")]
fn main() {
    pkg_config::probe_library("javascriptcoregtk-4.1").unwrap();
}

#[cfg(target_os = "windows")]
fn main() {
    // if GNU
    if !cfg!(target_env = "msvc")
    {
        panic!("Only MSVC is supported on Windows");
    }

    //println!("cargo:rustc-link-search=native={}", format!("{}/lib/win64", std::env::current_dir().unwrap().to_str().unwrap()));
    println!("cargo:rustc-link-lib=JavaScriptCore");
    //sprintln!("cargo:rustc-link-lib=jsc");
    println!("cargo:rustc-link-lib=sicudt");
    println!("cargo:rustc-link-lib=sicuin");
    println!("cargo:rustc-link-lib=sicuio");
    println!("cargo:rustc-link-lib=sicutu");
    println!("cargo:rustc-link-lib=sicuuc");
    println!("cargo:rustc-link-lib=WTF");
    println!("cargo:rustc-link-lib=winmm");
    println!("cargo:rustc-link-lib=shell32");
}
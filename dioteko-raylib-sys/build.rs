use std::fs;

fn main() -> std::io::Result<()> {
    // Build raylib with cmake
    let dst = cmake::Config::new("./lib/raylib")
        .very_verbose(true)
        .build();

    // Use prebuilt bindings.rs
    #[cfg(target_os = "windows")]
    fs::copy("src/bindings/windows_bindings.rs", "src/bindings.rs")?;
    #[cfg(target_os = "macos")]
    fs::copy("src/bindings/macos_bindings.rs", "src/bindings.rs")?;

    #[cfg(target_os = "windows")]
    println!("cargo:rustc-link-search=native={}\\lib", dst.display());
    #[cfg(not(target_os = "windows"))]
    println!("cargo:rustc-link-search=native={}/lib", dst.display());

    println!("cargo:rustc-link-lib=static=raylib");

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=dylib=winmm");
        println!("cargo:rustc-link-lib=dylib=gdi32");
        println!("cargo:rustc-link-lib=dylib=user32");
        println!("cargo:rustc-link-lib=dylib=shell32");
    }
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-link-search=native=/usr/local/lib");
        println!("cargo:rustc-link-lib=framework=OpenGL");
        println!("cargo:rustc-link-lib=framework=Cocoa");
        println!("cargo:rustc-link-lib=framework=IOKit");
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreVideo");
    }

    Ok(())
}

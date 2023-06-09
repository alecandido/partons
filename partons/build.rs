#[cfg(feature = "lhapdf")]
fn main() {
    let lhapdf = pkg_config::Config::new()
        .atleast_version("6")
        .probe("lhapdf")
        .unwrap();

    let mut build = cxx_build::bridge("src/engine/lhapdf.rs");

    for include_path in lhapdf.include_paths {
        build.include(include_path);
    }

    build
        .flag_if_supported("-std=c++11")
        .compile("lhapdf-rust-cxx-bridge");

    for lib_path in lhapdf.link_paths {
        println!("cargo:rustc-link-search={}", lib_path.to_str().unwrap());
    }

    for lib in lhapdf.libs {
        println!("cargo:rustc-link-lib=static={lib}");
    }

    println!("cargo:rerun-if-changed=src/engine/lhapdf.rs");
    println!("cargo:rerun-if-changed=include/wrappers.hpp");
}

#[cfg(not(feature = "lhapdf"))]
fn main() {}

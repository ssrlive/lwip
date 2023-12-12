use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

fn sdk_include_path_for(sdk: &str) -> String {
    // sdk path find by `xcrun --sdk {iphoneos|macosx} --show-sdk-path`
    let output = Command::new("xcrun")
        .arg("--sdk")
        .arg(sdk)
        .arg("--show-sdk-path")
        .output()
        .expect("failed to execute xcrun");

    let inc_path = Path::new(String::from_utf8_lossy(&output.stdout).trim()).join("usr/include");

    inc_path.to_str().expect("invalid include path").to_string()
}

fn sdk_include_path() -> Option<String> {
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    match os.as_str() {
        "ios" => {
            if arch == "x86_64" {
                Some(sdk_include_path_for("iphonesimulator"))
            } else {
                Some(sdk_include_path_for("iphoneos"))
            }
        }
        "macos" => Some(sdk_include_path_for("macosx")),
        _ => None,
    }
}

fn compile_lwip() {
    println!("cargo:rerun-if-changed=old-src/core");
    let mut build = cc::Build::new();
    build
        .file("old-src/core/init.c")
        .file("old-src/core/def.c")
        // .file("old-src/core/dns.c")
        .file("old-src/core/inet_chksum.c")
        .file("old-src/core/ip.c")
        .file("old-src/core/mem.c")
        .file("old-src/core/memp.c")
        .file("old-src/core/netif.c")
        .file("old-src/core/pbuf.c")
        .file("old-src/core/raw.c")
        // .file("old-src/core/stats.c")
        // .file("old-src/core/sys.c")
        .file("old-src/core/tcp.c")
        .file("old-src/core/tcp_in.c")
        .file("old-src/core/tcp_out.c")
        .file("old-src/core/timeouts.c")
        .file("old-src/core/udp.c")
        // .file("old-src/core/ipv4/autoip.c")
        // .file("old-src/core/ipv4/dhcp.c")
        // .file("old-src/core/ipv4/etharp.c")
        .file("old-src/core/ipv4/icmp.c")
        // .file("old-src/core/ipv4/igmp.c")
        .file("old-src/core/ipv4/ip4_frag.c")
        .file("old-src/core/ipv4/ip4.c")
        .file("old-src/core/ipv4/ip4_addr.c")
        // .file("old-src/core/ipv6/dhcp6.c")
        // .file("old-src/core/ipv6/ethip6.c")
        .file("old-src/core/ipv6/icmp6.c")
        // .file("old-src/core/ipv6/inet6.c")
        .file("old-src/core/ipv6/ip6.c")
        .file("old-src/core/ipv6/ip6_addr.c")
        .file("old-src/core/ipv6/ip6_frag.c")
        // .file("old-src/core/ipv6/mld6.c")
        .file("old-src/core/ipv6/nd6.c")
        .file("old-src/custom/sys_arch.c")
        .file("src/api/err.c")
        .include("old-src/custom")
        .include("old-src/include")
        .warnings(false)
        .flag_if_supported("-Wno-everything");
    if let Some(sdk_include_path) = sdk_include_path() {
        build.include(sdk_include_path);
    }
    build.debug(true);
    build.compile("liblwip.a");
}

fn generate_lwip_bindings() {
    println!("cargo:rustc-link-lib=lwip");
    // println!("cargo:rerun-if-changed=old-src/custom/wrapper.h");
    println!("cargo:include=old-src/include");

    let sdk_include_path = sdk_include_path();

    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let mut builder = bindgen::Builder::default()
        .header("old-src/custom/wrapper.h")
        .clang_arg("-I./old-src/include")
        .clang_arg("-I./old-src/custom")
        .clang_arg("-Wno-everything")
        .layout_tests(false)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()));
    if arch == "aarch64" && os == "ios" {
        // https://github.com/rust-lang/rust-bindgen/issues/1211
        builder = builder.clang_arg("--target=arm64-apple-ios");
    }
    if let Some(sdk_include_path) = sdk_include_path {
        builder = builder.clang_arg(format!("-I{}", sdk_include_path));
    }

    if os == "windows" {
        builder = builder.size_t_is_usize(false);
    }

    let bindings = builder.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

fn main() {
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    println!("cargo:warning=host os {}", os);
    compile_lwip();
    generate_lwip_bindings();
    println!("cargo:rerun-if-changed=build.rs");
}

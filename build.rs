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
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=contrib");
    let mut build = cc::Build::new();

    let src = "contrib/examples/example_app/lwipcfg.h.example";
    std::fs::copy(src, "rust/lwipcfg.h").unwrap();

    let os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    build.include("rust");
    build.include("src/include");
    if os == "windows" {
        build.include("contrib/ports/win32/include");
    } else {
        build.include("contrib/ports/unix/port/include");
    }

    if os == "windows" {
        // lwipcontribportwindows_SRCS
        build.file("contrib/ports/win32/sys_arch.c");
        build.file("contrib/ports/win32/sio.c");
    } else {
        // lwipcontribportunix_SRCS
        build.file("contrib/ports/unix/port/sys_arch.c");
    }

    // lwipcore_SRCS
    build
        .file("src/core/init.c")
        .file("src/core/def.c")
        .file("src/core/dns.c")
        .file("src/core/inet_chksum.c")
        .file("src/core/ip.c")
        .file("src/core/mem.c")
        .file("src/core/memp.c")
        .file("src/core/netif.c")
        .file("src/core/pbuf.c")
        .file("src/core/raw.c")
        .file("src/core/stats.c")
        .file("src/core/sys.c")
        .file("src/core/altcp.c")
        .file("src/core/altcp_alloc.c")
        .file("src/core/altcp_tcp.c")
        .file("src/core/tcp.c")
        .file("src/core/tcp_in.c")
        .file("src/core/tcp_out.c")
        .file("src/core/timeouts.c")
        .file("src/core/udp.c");

    // lwipcore4_SRCS
    build
        .file("src/core/ipv4/acd.c")
        .file("src/core/ipv4/autoip.c")
        .file("src/core/ipv4/dhcp.c")
        .file("src/core/ipv4/etharp.c")
        .file("src/core/ipv4/icmp.c")
        .file("src/core/ipv4/igmp.c")
        .file("src/core/ipv4/ip4_frag.c")
        .file("src/core/ipv4/ip4.c")
        .file("src/core/ipv4/ip4_addr.c");

    // lwipcore6_SRCS
    build
        .file("src/core/ipv6/dhcp6.c")
        .file("src/core/ipv6/ethip6.c")
        .file("src/core/ipv6/icmp6.c")
        .file("src/core/ipv6/inet6.c")
        .file("src/core/ipv6/ip6.c")
        .file("src/core/ipv6/ip6_addr.c")
        .file("src/core/ipv6/ip6_frag.c")
        .file("src/core/ipv6/mld6.c")
        .file("src/core/ipv6/nd6.c");

    // APIFILES: The files which implement the sequential and socket APIs.
    // lwipapi_SRCS
    build
        .file("src/api/api_lib.c")
        .file("src/api/api_msg.c")
        .file("src/api/err.c")
        .file("src/api/if_api.c")
        .file("src/api/netbuf.c")
        .file("src/api/netdb.c")
        .file("src/api/netifapi.c")
        .file("src/api/sockets.c")
        .file("src/api/tcpip.c");

    // Files implementing various generic network interface functions
    // lwipnetif_SRCS
    build
        .file("src/netif/ethernet.c")
        .file("src/netif/bridgeif.c")
        .file("src/netif/bridgeif_fdb.c")
        .file("src/netif/slipif.c"); // (NOT ${LWIP_EXCLUDE_SLIPIF})

    // 6LoWPAN
    // lwipsixlowpan_SRCS
    build
        .file("src/netif/lowpan6_common.c")
        .file("src/netif/lowpan6.c")
        .file("src/netif/lowpan6_ble.c")
        .file("src/netif/zepif.c");

    // PPP
    // lwipppp_SRCS
    build
        .file("src/netif/ppp/auth.c")
        .file("src/netif/ppp/ccp.c")
        .file("src/netif/ppp/chap-md5.c")
        .file("src/netif/ppp/chap_ms.c")
        .file("src/netif/ppp/chap-new.c")
        .file("src/netif/ppp/demand.c")
        .file("src/netif/ppp/eap.c")
        .file("src/netif/ppp/ecp.c")
        .file("src/netif/ppp/eui64.c")
        .file("src/netif/ppp/fsm.c")
        .file("src/netif/ppp/ipcp.c")
        .file("src/netif/ppp/ipv6cp.c")
        .file("src/netif/ppp/lcp.c")
        .file("src/netif/ppp/magic.c")
        .file("src/netif/ppp/mppe.c")
        .file("src/netif/ppp/multilink.c")
        .file("src/netif/ppp/ppp.c")
        .file("src/netif/ppp/pppapi.c")
        .file("src/netif/ppp/pppcrypt.c")
        .file("src/netif/ppp/pppoe.c")
        .file("src/netif/ppp/pppol2tp.c")
        .file("src/netif/ppp/pppos.c")
        .file("src/netif/ppp/upap.c")
        .file("src/netif/ppp/utils.c")
        .file("src/netif/ppp/vj.c")
        .file("src/netif/ppp/polarssl/arc4.c")
        .file("src/netif/ppp/polarssl/des.c")
        .file("src/netif/ppp/polarssl/md4.c")
        .file("src/netif/ppp/polarssl/md5.c")
        .file("src/netif/ppp/polarssl/sha1.c");

    build.warnings(false).flag_if_supported("-Wno-everything");

    if let Some(sdk_include_path) = sdk_include_path() {
        build.include(sdk_include_path);
    }
    build.debug(true);
    build.compile("liblwip.a");
}

fn generate_lwip_bindings() {
    println!("cargo:rustc-link-lib=lwip");
    // println!("cargo:include=src/include");

    let sdk_include_path = sdk_include_path();

    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let mut builder = bindgen::Builder::default()
        .header("src/include/lwip/init.h")
        .header("src/include/lwip/timeouts.h")
        .header("src/include/lwip/netif.h")
        .header("src/include/lwip/tcp.h")
        .header("src/include/lwip/udp.h")
        .header("src/include/lwip/ip_addr.h")
        .clang_arg("-I./src/include")
        .clang_arg("-I./contrib")
        .clang_arg("-I./rust")
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
        builder = builder.clang_arg("-I./contrib/ports/win32/include");
    } else {
        builder = builder.clang_arg("-I./contrib/ports/unix/port/include");
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

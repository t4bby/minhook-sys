use std::{env, path::Path};

fn main() {
    if env::var("CARGO_CFG_WINDOWS").is_err() {
        panic!("MinHook only supports Windows");
    }

    let hde = match env::var("CARGO_CFG_TARGET_ARCH").unwrap().as_str() {
        "x86" => "hde/hde32.c",
        "x86_64" => "hde/hde64.c",
        _ => panic!("only x86 and x86_64 architectures are supported"),
    };

    let srcs = Path::new("minhook/src");
    cc::Build::new()
        .file(srcs.join("buffer.c"))
        .file(srcs.join("hook.c"))
        .file(srcs.join("trampoline.c"))
        .file(srcs.join(hde))
        .compile("MinHook");
}
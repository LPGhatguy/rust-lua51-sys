extern crate cc;
extern crate bindgen;

use std::{
    env,
    path::Path,
};

fn main() {
    let bindings_file = Path::new(&env::var("OUT_DIR").unwrap()).join("lua.rs");

    if cfg!(feature = "dynamic") && !cfg!(feature = "static") {
        println!("cargo:rustc-link-lib=dylib=lua51");
    } else if cfg!(feature = "static") && !cfg!(feature = "dynamic") {
        cc::Build::new()
            .file("lua/src/lapi.c")
            .file("lua/src/lauxlib.c")
            .file("lua/src/lbaselib.c")
            .file("lua/src/lcode.c")
            .file("lua/src/ldblib.c")
            .file("lua/src/ldebug.c")
            .file("lua/src/ldo.c")
            .file("lua/src/ldump.c")
            .file("lua/src/lfunc.c")
            .file("lua/src/lgc.c")
            .file("lua/src/linit.c")
            .file("lua/src/liolib.c")
            .file("lua/src/llex.c")
            .file("lua/src/lmathlib.c")
            .file("lua/src/lmem.c")
            .file("lua/src/loadlib.c")
            .file("lua/src/lobject.c")
            .file("lua/src/lopcodes.c")
            .file("lua/src/loslib.c")
            .file("lua/src/lparser.c")
            .file("lua/src/lstate.c")
            .file("lua/src/lstring.c")
            .file("lua/src/lstrlib.c")
            .file("lua/src/ltable.c")
            .file("lua/src/ltablib.c")
            .file("lua/src/ltm.c")
            .file("lua/src/lua.c")
            .file("lua/src/luac.c")
            .file("lua/src/lundump.c")
            .file("lua/src/lvm.c")
            .file("lua/src/lzio.c")
            .file("lua/src/print.c")
            .include("lua/src")
            .compile("liblua.a");
    } else {
        panic!("Please enable exactly one of these features: dynamic, static");
    }

    bindgen::builder()
        .header("src/helper.h")
        .clang_arg("-Ilua/src")
        .whitelist_type("luaL?_.*")
        .whitelist_function("luaL?_.*")
        .whitelist_var("LUA_.*")
        .generate()
        .expect("Could not generate Lua bindings")
        .write_to_file(bindings_file)
        .expect("Could not write bindings file");
}
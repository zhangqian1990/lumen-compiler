use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=src/ffi/cpp");
    
    // 编译C++代码
    let mut build = cc::Build::new();
    
    // 设置C++17标准
    build.cpp(true)
        .std("c++17")
        .warnings(true)
        .extra_warnings(true);
    
    // 添加调试或优化标志
    if env::var("PROFILE").unwrap() == "release" {
        build.opt_level(3);
    } else {
        build.debug(true);
    }
    
    // 添加C++源文件
    build.file("src/ffi/cpp/parser.cpp");
    
    // 检查操作系统和架构相关设置
    let target = env::var("TARGET").unwrap();
    if target.contains("linux") {
        build.flag("-pthread");
    } else if target.contains("windows") {
        build.flag("/EHsc");
    }
    
    // 尝试启用SIMD优化
    if target.contains("x86_64") {
        if target.contains("linux") || target.contains("darwin") {
            build.flag("-mavx2");
        } else if target.contains("windows") {
            build.flag("/arch:AVX2");
        }
    }
    
    // 编译为静态库
    build.compile("lumen_cpp");
    
    // 设置库的链接目录
    println!("cargo:rustc-link-search=native={}", env::var("OUT_DIR").unwrap());
    
    // 链接系统库
    if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else if target.contains("darwin") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("windows") {
        println!("cargo:rustc-link-lib=dylib=msvcrt");
    }
    
    // 生成Rust绑定
    generate_bindings();
}

fn generate_bindings() {
    let bindings = bindgen::Builder::default()
        .header("src/ffi/cpp/parser.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("无法生成绑定");
    
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("无法写入绑定");
} 
use std::env;
use std::path::PathBuf;

fn main() {
    // 告诉cargo在C++文件改变时重新编译
    println!("cargo:rerun-if-changed=src/ffi/cpp");

    // 获取环境变量
    let out_dir = env::var("OUT_DIR").ok();
    println!("OUT_DIR = {:?}", out_dir);
    let opt_level = env::var("OPT_LEVEL").ok();
    println!("OPT_LEVEL = {:?}", opt_level);
    let target = env::var("TARGET").ok();
    println!("TARGET = {:?}", target);
    let host = env::var("HOST").ok();
    println!("HOST = {:?}", host);

    // 仅在生产环境中编译C++代码，开发环境临时跳过
    let compile_cpp = false; // 暂时禁用C++编译

    if compile_cpp {
        // 编译C++代码
        let mut build = cc::Build::new();
        
        // 配置编译选项
        build.cpp(true)
             .include("src/ffi/cpp")
             .include("vendor/include");
        
        // 优化配置
        if let Some(opt_level) = opt_level {
            if opt_level != "0" {
                build.opt_level(3);
            } else {
                build.debug(true);
            }
        }
        
        // 添加源文件
        build.file("src/ffi/cpp/parser.cpp");
        
        // 平台特定选项
        if cfg!(target_os = "linux") {
            build.flag("-pthread");
        } else if cfg!(target_os = "windows") {
            build.flag("/EHsc");
        }
        
        // 架构特定优化
        if cfg!(target_arch = "x86_64") {
            build.flag("-mavx2");
        } else if cfg!(target_os = "windows") {
            build.flag("/arch:AVX2");
        }
        
        // 编译库
        build.compile("lumen_cpp");
    }

    // 生成Rust绑定 - 同样暂时禁用
    if false {
        let bindings = bindgen::Builder::default()
            .header("src/ffi/c/bindings.h")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .generate()
            .expect("生成绑定失败");
        
        // 将绑定写入output目录
        let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
        bindings
            .write_to_file(out_path.join("bindings.rs"))
            .expect("写入绑定失败");
    }
} 
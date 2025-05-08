use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::path::Path;
use log::{debug, info};

// 定义外部C++函数接口
#[link(name = "lumen_cpp")]
extern "C" {
    // 解析器相关函数
    fn cpp_parse_js(source: *const c_char, length: c_int) -> *mut c_char;
    fn cpp_parse_ts(source: *const c_char, length: c_int) -> *mut c_char;
    fn cpp_parse_jsx(source: *const c_char, length: c_int) -> *mut c_char;
    fn cpp_parse_tsx(source: *const c_char, length: c_int) -> *mut c_char;
    
    // 代码生成相关函数
    fn cpp_generate_code(ir_json: *const c_char, minify: c_int, target: *const c_char) -> *mut c_char;
    fn cpp_generate_wasm(ir_json: *const c_char, opts_json: *const c_char) -> *mut c_char;
    
    // 优化相关函数
    fn cpp_optimize_ir(ir_json: *const c_char, level: c_int) -> *mut c_char;
    
    // 内存管理函数
    fn cpp_free_string(ptr: *mut c_char);
}

// 安全封装C++解析器
pub struct CppParser;

impl CppParser {
    pub fn parse_js(source: &str) -> Result<String, String> {
        unsafe {
            let c_source = CString::new(source).map_err(|e| format!("无法创建C字符串: {}", e))?;
            let result_ptr = cpp_parse_js(c_source.as_ptr(), source.len() as c_int);
            
            if result_ptr.is_null() {
                return Err("C++解析器返回空指针".to_string());
            }
            
            let result = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
            cpp_free_string(result_ptr);
            
            Ok(result)
        }
    }
    
    pub fn parse_ts(source: &str) -> Result<String, String> {
        unsafe {
            let c_source = CString::new(source).map_err(|e| format!("无法创建C字符串: {}", e))?;
            let result_ptr = cpp_parse_ts(c_source.as_ptr(), source.len() as c_int);
            
            if result_ptr.is_null() {
                return Err("C++解析器返回空指针".to_string());
            }
            
            let result = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
            cpp_free_string(result_ptr);
            
            Ok(result)
        }
    }
    
    pub fn parse_jsx(source: &str) -> Result<String, String> {
        unsafe {
            let c_source = CString::new(source).map_err(|e| format!("无法创建C字符串: {}", e))?;
            let result_ptr = cpp_parse_jsx(c_source.as_ptr(), source.len() as c_int);
            
            if result_ptr.is_null() {
                return Err("C++解析器返回空指针".to_string());
            }
            
            let result = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
            cpp_free_string(result_ptr);
            
            Ok(result)
        }
    }
    
    pub fn parse_tsx(source: &str) -> Result<String, String> {
        unsafe {
            let c_source = CString::new(source).map_err(|e| format!("无法创建C字符串: {}", e))?;
            let result_ptr = cpp_parse_tsx(c_source.as_ptr(), source.len() as c_int);
            
            if result_ptr.is_null() {
                return Err("C++解析器返回空指针".to_string());
            }
            
            let result = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
            cpp_free_string(result_ptr);
            
            Ok(result)
        }
    }
}

// 安全封装C++代码生成器
pub struct CppCodeGenerator;

impl CppCodeGenerator {
    pub fn generate_code(ir_json: &str, minify: bool, target: &str) -> Result<String, String> {
        unsafe {
            let c_ir_json = CString::new(ir_json).map_err(|e| format!("无法创建C字符串: {}", e))?;
            let c_target = CString::new(target).map_err(|e| format!("无法创建C字符串: {}", e))?;
            let minify_int = if minify { 1 } else { 0 };
            
            let result_ptr = cpp_generate_code(c_ir_json.as_ptr(), minify_int, c_target.as_ptr());
            
            if result_ptr.is_null() {
                return Err("C++代码生成器返回空指针".to_string());
            }
            
            let result = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
            cpp_free_string(result_ptr);
            
            Ok(result)
        }
    }
    
    pub fn generate_wasm(ir_json: &str, options_json: &str) -> Result<String, String> {
        unsafe {
            let c_ir_json = CString::new(ir_json).map_err(|e| format!("无法创建C字符串: {}", e))?;
            let c_options = CString::new(options_json).map_err(|e| format!("无法创建C字符串: {}", e))?;
            
            let result_ptr = cpp_generate_wasm(c_ir_json.as_ptr(), c_options.as_ptr());
            
            if result_ptr.is_null() {
                return Err("C++ WebAssembly生成器返回空指针".to_string());
            }
            
            let result = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
            cpp_free_string(result_ptr);
            
            Ok(result)
        }
    }
}

// 安全封装C++优化器
pub struct CppOptimizer;

impl CppOptimizer {
    pub fn optimize(ir_json: &str, level: i32) -> Result<String, String> {
        unsafe {
            let c_ir_json = CString::new(ir_json).map_err(|e| format!("无法创建C字符串: {}", e))?;
            
            let result_ptr = cpp_optimize_ir(c_ir_json.as_ptr(), level);
            
            if result_ptr.is_null() {
                return Err("C++优化器返回空指针".to_string());
            }
            
            let result = CStr::from_ptr(result_ptr).to_string_lossy().into_owned();
            cpp_free_string(result_ptr);
            
            Ok(result)
        }
    }
} 
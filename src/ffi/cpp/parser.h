#ifndef LUMEN_PARSER_H
#define LUMEN_PARSER_H

#ifdef __cplusplus
extern "C" {
#endif

// 解析器相关函数
char* cpp_parse_js(const char* source, int length);
char* cpp_parse_ts(const char* source, int length);
char* cpp_parse_jsx(const char* source, int length);
char* cpp_parse_tsx(const char* source, int length);

// 代码生成相关函数
char* cpp_generate_code(const char* ir_json, int minify, const char* target);
char* cpp_generate_wasm(const char* ir_json, const char* opts_json);

// 优化相关函数
char* cpp_optimize_ir(const char* ir_json, int level);

// 内存管理函数
void cpp_free_string(char* ptr);

#ifdef __cplusplus
}
#endif

#endif // LUMEN_PARSER_H 
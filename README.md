# Rust ACM/ICPC 模板

## 多文件打包

基于[rqdmap/rust-bundler](https://github.com/rqdmap/rust-bundler)提供的Rust功能库, 使用Cargo构建脚本即可将项目下的多个文件打包成单一Rust文件, 便于在OJ上提交代码.

为了使用打包功能, 维护如下所示的项目文件结构:

```
src
├── basic
│   ├── io.rs
│   └── mod.rs
├── bin
│   └── main.rs
├── lib.rs
└── math
    ├── mod.rs
    └── number_theory.rs
```

项目主体是一个lib项目, 拥有lib.rs; 在bin/main.rs中编写可执行文件的相关代码, 其中按需包含lib库的各个模块.

使用`cargo`编译/运行时即可在项目顶级目录下打包生成`bundle.rs`文件.

## 模板内容

TBD









//! 低级启动汇编代码（Boot Assembly Code）
//!
//! 这段汇编用于程序的最初启动阶段，设置栈指针，并跳转到 Rust 的主入口函数 `rust_main`。
//! 同时定义了一个用于内核或裸机程序的静态栈空间。
//!
//! 主要功能包括：
//! 1. 设置栈指针 `sp`。
//! 2. 调用 Rust 层的主函数 `rust_main`。
//! 3. 定义 `.bss` 段的栈空间。
//!
//! 注意：这是裸机或操作系统内核开发中的启动代码，不依赖标准库。

use core::arch::global_asm;

global_asm!(
    r#"
    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top
    call rust_main

    .section .bss.stack
    .globl boot_stack
boot_stack:
    .space 4096 * 64
    .globl boot_stack_top
boot_stack_top:
"#
);

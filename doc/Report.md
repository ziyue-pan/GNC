

# 序言

**GNC**是**GNC is Not C**的递归表达。我们尝试构造了一种贴近C，但更加现代化的语法。它使用rust作为主体开发语言，[pest]([pest. The Elegant Parser](https://pest.rs/))作为parser generator，使用了LLVM作为中间表达和后端处理，并使用[inkwell]([TheDan64/inkwell: It's a New Kind of Wrapper for Exposing LLVM (Safely) (github.com)](https://github.com/TheDan64/inkwell))作为LLVM与rust的Binding。

由于使用LLVM，理论上我们可以最终编译到所有LLVM支持的汇编语言。在开发过程中，我们默认编译到x86 (amd64)、RISC-V (rv64) 和 WebAssembly (wasm64)三种汇编语言，来做最终的运行时检查。





# 项目背景

## 为什么选择rust

Rust是近些年产生的一个





## 为什么选择LLVM







## 为什么是GNC而不是C







## 技术选型







# 语言设计











# 词法分析和语法分析



这就是词法分析？

# 语义分析

这就是语义分析？





# 优化考虑

## 使用语法规则处理优先级







## 错误流处理









## 类型系统





# 代码生成





# 测试案例

这就是测试案例？


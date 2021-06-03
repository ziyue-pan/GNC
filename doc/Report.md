

# 序言

**GNC**是**GNC is Not C**的递归表达。我们尝试构造了一种贴近C，但更加现代化的语法。它使用rust作为主体开发语言，[pest]([pest. The Elegant Parser](https://pest.rs/))作为parser generator，使用了LLVM作为中间表达和后端处理，并使用[inkwell]([TheDan64/inkwell: It's a New Kind of Wrapper for Exposing LLVM (Safely) (github.com)](https://github.com/TheDan64/inkwell))作为LLVM与Rust的Binding。

由于使用LLVM，理论上我们可以最终编译到所有LLVM支持的汇编语言。在开发过程中，我们默认编译到x86 (amd64)、RISC-V (rv64) 和 WebAssembly (wasm64)三种汇编语言，来做最终的运行时检查。

TODO: 成果



# 项目背景

## 为什么选择rust

Rust是近些年诞生的一门编译型语言。它的设计准则是“安全、并发、实用”。

Rust的作用领域是系统级编程，在这个领域有老大哥C和C++，但这两种语言碍于当时编译技术的落后和历史路径的依赖，不能完美地保证内存安全。此外，带有GC (Garbage Collection)的语言又难免需要运行时环境的支持，在GC时还难免引入不小的性能开销。

Rust通过严格的编译器检查和静态分析技术 (CFI、Lifetime、Ownership等)，使得程序员必须遵守一定的代码准则 (be explicit)，才能使得Rust代码通过编译。这样的特性显著地提高了Rust的学习成本，但也让Rust代码的开发和维护成本降到了最低。

Rust有着良好的工具链支持，包括现代化的构建工具和包管理系统 (Cargo和[crates.io](https://crates.io))、美观实用的注释文档生成框架 (Rustdoc)、强大的IDE (Clion的Rust插件)以及庞大完善的社区体系 (Reddit, Stackoverflow, [Rust Forum](https://users.rust-lang.org))。

Rust的语法特性在诸多语言中可以算是最为严谨和强大的，它在支持函数式编程、元编程和反射等诸多特性上，保证了语言的高度安全性。其他诸如动态引用、智能指针等语法糖设计地恰到好处，也让Rust这门语言看上起十分优雅。

基于以上诸多Rust的优点和兴趣的考虑，我们最终选择了Rust作为主体开发语言。

## 为什么选择LLVM







## 为什么是GNC而不是C







## 技术选型

GNC依然使用了经典的编译器架构：前端 (parser)和后端 (codegen)的设计。

前端使用了[pest](https://pest.rs)库作为parser generator。它使用的是类PEG (Parsing Expression Grammar)描述，可以按照写明的语法规则生成一个与Rust深度耦合的Parser。这个Parser将代码的输入解析成Parser Tree，再自己编写遍历Parser Tree的规则生成到AST。

后端使用了LLVM工具链，并且用[inkwell](https://github.com/TheDan64/inkwell)作为LLVM和Rust的Binding。

在Parser Tree和AST可视化上，我们为了视觉效果，选择了[AntV G6](https://g6.antv.vision/zh)可视化引擎，并且将Parse和遍历AST的过程从Rust编译到WebAssembly，部署到Github的静态前端页面上: [GNC Compiler Online](https://ziyuepan.tech/GNC/)。

TODO: HLVM



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


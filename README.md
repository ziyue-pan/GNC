# GNC

[![Build And Test](https://github.com/PAN-Ziyue/GNC/workflows/CI/badge.svg?event=push)](https://github.com/PAN-Ziyue/GNC/actions?workflow=CI)


**GNC** is **N**ot **C**. It is intended for a better and more effective c language.

## Grammar Features

- [x] int main() ...
- [x] declare local int variable
- [x] unary operation  
- [x] binary operation
- [x] conditional statement
- [x] loop statement
- [x] scope 
- [x] function
- [x] global variable
- [x] more types
- [x] cast expression
- [x] pointer
- [x] string
- [x] `scanf()` and `printf

## Other Features

- [x] Pest as frontend
- [x] LLVM as backend
- [x] Parse tree and AST visualization using AntV
- [x] HLVM (High Level Virtual Machine Interpreter)
- [x] Full compiler capabilities in the browser using WASM

## Build

### GNC Cli

```bash
$ cargo build --package GNC --bin GNC
```

### GNC Online

```bash
$ cd forntend
$ yarn
$ yarn build
```

## Test

### Environment

```bash
# install zx
$ npm i -g zx
```

### Run

```bash
$ cd test
$ zx ./minidecaf-test.mjs
```

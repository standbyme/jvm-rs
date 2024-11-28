# jvm-rs
[![Codecov](https://img.shields.io/codecov/c/github/standbyme/jvm-rs.svg?style=flat-square)](https://codecov.io/gh/standbyme/jvm-rs)
[![GitHub license](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](https://github.com/standbyme/jvm-rs/) 
[![](https://img.shields.io/gitter/room/jvm-rs/main.js.svg)](https://gitter.im/jvm-rs/main)

A JVM on Rust under development.

Strive to achieve 
- **Clear** structure.
- Well **tested**
- Minimal Unsafe(**Only** [src/util/converter.rs](https://github.com/standbyme/jvm-rs/blob/master/src/util/converter.rs) has unsafe code)
- Minimal Mutable
- Functional Programming

Now it support **Arithmetic Operation, Control Flow and println function call**. It's very easy to understand and follow.

Welcome Star : )

# Introduction
jvm-rs is a toy JVM (which is far from complete) programmed in Rust inspired by [jvm.go](https://github.com/zxh0/jvm.go). The main purpose of this project is learning Rust and the JVM. So the number one goal of the project is readability of code. The basic idea is to just implement the core JVM, and use `rt.jar` (from OpenJDK) as its class library.

# My dev environment
  * Ubuntu 18.04
  * Java 1.8.0_172
  * Rust 1.27.0

# Get and Build jvm-rs
Ensure your Java version is 1.8.0_172 and JAVA_HOME env was set
```sh
git clone https://github.com/standbyme/jvm-rs.git
cd jvm-rs
cargo test
```

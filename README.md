# jvm-rs
[![Linux Build Status](https://img.shields.io/travis/standbyme/jvm-rs/master.svg?label=Linux%20build)](https://travis-ci.org/standbyme/jvm-rs)

A JVM on Rust under development.

Strive to achieve 
- **Clear** structure.
- Well **tested**
- Minimal(or No) Unsafe
- Minimal Mutable
- Functional Programming

Now it is able to parse ClassFile and it's very easy to understand and follow.

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
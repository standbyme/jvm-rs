# jvm-rs
A JVM written in Rust.

# Introduction
jvm-rs is a toy JVM (which is far from complete) programmed in Rust inspired by [jvm.go](https://github.com/zxh0/jvm.go). The main purpose of this project is learning Rust and the JVM. So the number one goal of the project is readability of code. The basic idea is to just implement the core JVM, and use `rt.jar` (from OpenJDK) as its class library.

# My dev environment
  * Ubuntu 18.04
  * Java 1.8.0_172
  * Rust 1.26.1

# Get and Build jvm-rs
Ensure your Java version is 1.8.0_172 and JAVA_HOME env was set
```sh
git clone https://github.com/standbyme/jvm-rs.git
```
**Open JAVA_HOME/jre/lib/rt.jar and Copy rt/java/lang/Object.class to jvm-rs**
```sh
cd jvm-rs
cargo run
```
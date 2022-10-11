# graal-rust-llvm-poc
Playing around with FFI bindings for polyglot.h


Invalid bitcode: recursive PointerType
====

* Make sure GraalVM 22.2 is your selected JDK (e.g. using sdkman)
* `cargo build --release`
* `cd demo`
* `cargo rustc --release -- --emit=llvm-bc`
* `LD_LIBRARY_PATH=$(lli --print-toolchain-api-paths LD_LIBRARY_PATH) lli --jvm --lib $(rustc --print sysroot)/lib/libstd-* --lib ../target/release/libpolyglot_rs.so target/release/deps/demo-<custom-hash>.bc`
* prints `Invalid bitcode: recursive PointerType`

use std::process::Command;

fn main() {
    // Tell rustc where to find graalvm-llvm lib
    let graalvm_lib_path = Command::new("lli")
        .args(["--print-toolchain-api-paths", "LD_LIBRARY_PATH"])
        .output()
        .expect(
            "Failed to execute `lli`, make sure that GraalVM is installed and `lli` is on the PATH",
        )
        .stdout;

    let graalvm_lib_path = String::from_utf8(graalvm_lib_path).expect("Failed to read lli output.");

    println!("cargo:rustc-link-search={graalvm_lib_path}");
    println!("cargo:rustc-link-lib=graalvm-llvm");
}

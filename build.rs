/* Copyright (C) 2018 Olivier Goffart <ogoffart@woboq.com>
Permission is hereby granted, free of charge, to any person obtaining a copy of this software and
associated documentation files (the "Software"), to deal in the Software without restriction,
including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense,
and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:
The above copyright notice and this permission notice shall be included in all copies or substantial
portions of the Software.
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT
NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND
NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES
OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/
use std::io::Result;
use std::process::Command;

fn protobuf() -> Result<()> {
    prost_build::compile_protos(&["protobuf/LocalStorageProtocol.proto"], &["protobuf/"])?;
    Ok(())
}

fn main() {
    std::env::set_var("PROTOC", protobuf_src::protoc());
    protobuf().unwrap();
    let output = Command::new("git")
        .args(&["rev-parse", "HEAD"])
        .output()
        .unwrap();
    let git_hash = String::from_utf8(output.stdout).unwrap();
    println!("cargo:rustc-env=GIT_HASH={}", git_hash);

    // Print a warning when rustc is too old.
    if !version_check::is_min_version("1.48.0").unwrap_or(false) {
        if let Some(version) = version_check::Version::read() {
            panic!(
                "Crayfish requires Rust 1.48.0 or later.  You are using rustc {}",
                version
            );
        } else {
            panic!("Crayfish requires Rust 1.48.0 or later, but could not determine Rust version.");
        }
    }
    let mer_target_root = "";

    let mut cfg = cpp_build::Config::new();

    cfg.flag(&format!("--sysroot={}", mer_target_root));
    cfg.flag("-isysroot");
    cfg.flag(mer_target_root);
    cfg.build("src/main.rs");
}

/// XXX TODO UPDATE THIS:
/// This build script allows us to enable the `read_buf` language feature only
/// for Rust Nightly.
///
/// See the comment in lib.rs to understand why we need this.

#[rustversion::nightly]
fn setup_cfg_unstable_clippy() {
    println!("cargo:rustc-check-cfg=cfg(unstable_clippy)");
    println!("cargo:rustc-cfg=unstable_clippy");
}

#[rustversion::not(nightly)]
fn setup_cfg_unstable_clippy() {
    // XXX TODO REPEATED STATEMENT(S) HERE:
    println!("cargo:rustc-check-cfg=cfg(unstable_clippy)");
}

#[cfg_attr(feature = "read_buf", rustversion::not(nightly))]
fn setup_cfg_read_buf() {
    // XXX TODO REPEATED STATEMENT(S) HERE:
    println!("cargo:rustc-check-cfg=cfg(bench)");
    println!("cargo:rustc-check-cfg=cfg(read_buf)");
}

#[cfg(feature = "read_buf")]
#[rustversion::nightly]
fn setup_cfg_read_buf() {
    println!("cargo:rustc-check-cfg=cfg(bench)");
    println!("cargo:rustc-check-cfg=cfg(read_buf)");
    println!("cargo:rustc-cfg=read_buf");
}

fn main() {
    setup_cfg_unstable_clippy();

    setup_cfg_read_buf();
}

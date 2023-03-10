use bootloader_locator::locate_bootloader;
use std::path::Path;
use std::process::Command;

pub fn main() {
    //the .unwrap() is needed as the bootloader function may fail, as we are allowing multiple different bootloaders to be loaded.
    //in future, I may want read and re-write the source code of the locate_bootloader function (https://docs.rs/crate/bootloader-locator/0.0.4/source/src/lib.rs) as it isn't that complicated and removes some more dependencies (as this is supposed to be a relatively dep free project!).
    let bootloader_manifest = locate_bootloader("bootloader").unwrap();

    // [TODO] don't hardcore this and derive dynamically (as is done with the locate_bootloader fn)
    let kernel_binary = Path::new("target/x86_64-unknown-none/debug/cidi-os")
        .canonicalize()
        .unwrap();

    // the path to the root of this crate, set by cargo
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    // we know that the kernel lives in the parent directory
    let kernel_dir = manifest_dir.parent().unwrap();

    let kernel_manifest = kernel_dir.join("Cargo.toml");
    // use the same target folder for building the bootloader
    let target_dir = kernel_dir.join("target");
    // place the resulting disk image next to our kernel binary
    let out_dir = kernel_binary.parent().unwrap();

    // create a new build command; use the `CARGO` environment variable to
    // also support non-standard cargo versions without hard-coding
    let mut build_cmd = Command::new(env!("CARGO"));

    // pass the arguments
    build_cmd.arg("builder");
    build_cmd.arg("--kernel-manifest").arg(&kernel_manifest);
    build_cmd.arg("--kernel-binary").arg(&kernel_binary);
    build_cmd.arg("--target-dir").arg(&target_dir);
    build_cmd.arg("--out-dir").arg(&out_dir);

    // set the working directory - here .parent() should never return None, as the bootloader_manifest path will always have a parent and thus can't error, and it is safe to unwrap.
    let bootloader_dir = bootloader_manifest.parent().unwrap();
    build_cmd.current_dir(&bootloader_dir);

    // run the command
    let exit_status = build_cmd.status().unwrap();
    if !exit_status.success() {
        panic!("bootloader build failed");
    }
}

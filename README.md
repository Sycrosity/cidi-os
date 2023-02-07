`cidi-os`
==================

cidi-os stands for `Can I Do It OS`, as this is an attempt to build a simple enough operating system to be able to run rust programs at a basic level, using the [blog-os](https://os.phil-opp.com/) tutorial.

## Usage

as this os requires a blank, no std target, so you must install the `x86_64-unknown-none` rust target:

```bash
rustup target add x86_64-unknown-none
```

And if you don't have it already, install [`qemu`] using the [instructions on its website](https://www.qemu.org/download/).

To run this os, you will need to run a few extra steps vs a normal cargo project:

```bash
$ cargo build
$ cargo run -p boot
$ qemu-system-x86_64 -drive \
      format=raw,file=target/x86_64-unknown-none/debug/boot-uefi-cidi-os.img \
      -bios tools/OVMF-pure-efi.fd
```

alternatively (although this is a clunky alternative), [install cargo-make](https://github.com/sagiegurari/cargo-make) and run:
```bash
cargo make run
```

~~**HOWEVER** - for whatever reason, running qemu within an embedded terminal like the vs-code terminal doesn't always work and may just not open a window for the OS - try and run this on your computers inbuilt terminal.~~

-------

## License
Licensed under either of

 - Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
 - MIT license
   ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
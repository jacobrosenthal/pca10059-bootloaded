# pca10059-bootloaded

The dongle comes with a dfu bootloader and from nordic.

## prereqs

* [nrfdfu](https://github.com/ferrous-systems/nrfdfu-rs) `cargo install nrfdfu`

## flash

* press reset (the sideways button) button to enable bootloader
* `cargo run` or use vscode run button

Since its dfu bootloaded the dev profile is setup just like a release profile so you don't need `--release`

## to get the stock bootloader back if you lost it

Youll need a jlink programmer and the tagconnect spring connector (but you would have probably needed that to remove the bootloader in the first place).

* Download [7853.pca10059_bootloader_mbr_v1.0.1.hex](https://devzone.nordicsemi.com/f/nordic-q-a/40924/how-can-i-restore-the-original-bootloader-of-a-pca10059)
* `cargo install probe-rs-cli` [probe-rs-cli](https://github.com/probe-rs/probe-rs)
* `probe-rs-cli download --format hex 7853.pca10059_bootloader_mbr_v1.0.1.hex --chip-erase`

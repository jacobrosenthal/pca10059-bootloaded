# pca10059-bootloaded

The dongle comes with a dfu bootloader from nordic.

## prereqs

* dongle-flash `cargo install --git https://github.com/ferrous-systems/embedded-trainings-2020 dongle-flash --branch main`
* install python and `pip install nrfutil`

## flash

* press reset (the sideways button) button to enable bootloader
* `cargo run` or use vscode run button

Since its dfu bootloaded the dev profile is setup just like a release profile so you don't need `--release`

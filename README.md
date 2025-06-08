# Bevy + Anvil = We have OpenSCAD at home :)



https://github.com/user-attachments/assets/f54e3c64-4dec-4487-b530-bb263bd5ce93

Disclaimer: This is a demo, not production ready code.

## Usage

See `examples/editor.rs`

To run the editor with hot-reloading:

1. Follow instructions here to set up `dx` and `.cargo/config.toml`: https://github.com/TheBevyFlock/bevy_simple_subsecond_system/?tab=readme-ov-file#first-time-installation
2. Run `dx serve --hot-patch --features dynamic --example editor`

Using this library with hot patching requires `--features dynamic`, which links `anvil` dynamically instead of statically, because of [this issue](https://github.com/DioxusLabs/dioxus/issues/4237).

## Licensing

Anvil is licensed LGPLv2.1 and to the extent I own the copyright on the code in this repo you are free to use it under that, or the MIT or Apache2 licenses.

`examples/editor.rs` contains example code copied from Anvil to make a lego brick, and as such can probably only be used under the LGPLv2.1 license.

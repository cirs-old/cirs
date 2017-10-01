// Copyright 2017 Cirs Project Developers
//
// Licensed under the Apache License, Version 2.0, <LICENSE-APACHE or
// http://apache.org/licenses/LICENSE-2.0> or the MIT license <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

// No libcore, as we implement parts as needed.
#![cfg_attr(feature = "no_core", feature(no_core))]
#![cfg_attr(feature = "no_core", no_core)]
// We definitely don't depend on libstd.
// #![no_core] generally covers it for no_core environment.
#![cfg_attr(not(feature = "no_core"), no_std)]
// Rust language items
#![feature(lang_items)]

// TODO: Start no core implementation as needed.
#[cfg(feature = "no_core")]
mod core {}

// TODO: Move this around, aswell provide a proper impl.
#[lang = "panic_fmt"]
fn panic_fmt() -> ! {
    loop {}
}

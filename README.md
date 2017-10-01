cirs
===

A (**very early work in progress**) implementation of C standard library in the
[Rust Language].

**NOTE:** This crate is not intended to be directly depended on. Rather you
should depend on the system libc produced by the crate.

* [Features](#features)
* [Supported architectures](#supported-architectures)
    * [Gold Tier](#gold-tier)
    * [Silver Tier](#silver-tier)
    * [Bronze Tier](#bronze-tier)
* [License](#license)
    * [Contribution](#contribution)

# Features
* Zero C code. Zero C dependencies. `cirs` programs should still be able to
interoperate with or link to C code.
* Static and dynamic cstd library support.

# Supported architectures
Like Rust language support architecture, we use a 3 tier system.

## Gold Tier
The platforms are guaranteed to function. The following requirements are
necessary:
* Build successfully.
* Automated testing for the platform.
* Documentation for how to use and how to build the platform is available.

Target                     | Notes
:-------------------------:| -------------
`x86_64-unknown-linux-cirs`| 64 bit Linux

## Silver Tier
The platforms may or may not function. The following requirements are
necessary:
* Build successfully.

## Bronze Tier
The platform is being worked on and is in very early stages. As such there are
no guarantees provided.


# License
Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE] or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT] or http://opensource.org/licenses/MIT)

at your option.

## Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any
additional terms or conditions.

[Rust Language]: https://www.rust-lang.org/en-US/
[LICENSE-APACHE]: LICENSE-APACHE
[LICENSE-MIT]: LICENSE-MIT

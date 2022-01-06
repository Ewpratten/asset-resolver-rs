# Asset Resolver
[![Crates.io](https://img.shields.io/crates/v/asset-resolver)](https://crates.io/crates/asset-resolver) 
[![Docs.rs](https://docs.rs/asset-resolver/badge.svg)](https://docs.rs/asset-resolver) 
[![Build](https://github.com/Ewpratten/asset-resolver-rs/actions/workflows/build.yml/badge.svg)](https://github.com/Ewpratten/asset-resolver-rs/actions/workflows/build.yml)
[![Clippy](https://github.com/Ewpratten/asset-resolver-rs/actions/workflows/clippy.yml/badge.svg)](https://github.com/Ewpratten/asset-resolver-rs/actions/workflows/clippy.yml)
[![Audit](https://github.com/Ewpratten/asset-resolver-rs/actions/workflows/audit.yml/badge.svg)](https://github.com/Ewpratten/asset-resolver-rs/actions/workflows/audit.yml)

The **Asset Resolver** crate exists to allow programs to create resolvers that can convert arbitrary path strings into filesystem paths.

The goal of this crate is to provide a simple and extensible API for other crates to use to provide their own resolvers. Crates should then be able to chain together lists of various other resolvers to efficiently convert identifiers into usable absolute paths.

Example resolvers would be as follows:

- URI to local filesystem
- Windows to UNIX
- Network path to mountpoint
- Anything else

This crate provides the following resolvers:

- [`DefaultResolver`](./src/resolver.rs#L20)
  - Turns strings into literal paths
- [`NullResolver`](./src/resolver.rs#L40)
  - Always fails


//! This module contains the main code revolving around the resolver mechanism

use std::path::{PathBuf, Path};

/// The asset resolver trait. This should be implemented by any custom resolvers
pub trait AssetResolver {
    /// Resolve a path to an asset.
    ///
    /// The incoming path could be in any format. If it can be 
    /// resolved by the implementer, it should return `Some(PathBuf)`. 
    /// Otherwise, it should return `None`.
    ///
    /// Additional mechanisms may handle chaining resolvers to handle multiple 
    /// path formats, so don't worry about catching every case in one resolver.
    fn resolve(&self, path: &str) -> Option<PathBuf>;
}


/// The default resolver. This simply parses path strings using `Path::new()`
pub struct DefaultResolver;

impl AssetResolver for DefaultResolver {

    /// Directly resolves paths using Rust's built-in 
    /// [`Path`](https://doc.rust-lang.org/std/path/struct.Path.html) type.
    /// 
    /// # Example
    /// ```
    /// # use asset_resolver::{AssetResolver, DefaultResolver};
    /// let resolver = DefaultResolver;
    /// let path = resolver.resolve("/home/user/assets/test.png").unwrap();
    /// assert_eq!(path.to_str().unwrap(), "/home/user/assets/test.png");
    /// ```
    fn resolve(&self, path: &str) -> Option<PathBuf> {
        Path::new(path).to_owned().into()
    }
}

/// A resolver that always returns `None`.
pub struct NullResolver;

impl AssetResolver for NullResolver {

    /// Does not resolve paths. Just returns `None`.
    fn resolve(&self, _path: &str) -> Option<PathBuf> {
        None
    }
}
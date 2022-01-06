use std::path::PathBuf;

use crate::AssetResolver;

/// An asset resolver that chains sub-resolvers together to
/// attempt to resolve a path using multiple ordered methods.
pub struct ResolverChain {
    resolvers: Vec<Box<dyn AssetResolver>>,
}

impl ResolverChain {
    /// Construct a new resolver chain.
    ///
    /// # Example
    /// ```
    /// # use asset_resolver::*;
    /// let resolver = ResolverChain::new(vec![
    ///   Box::new(NullResolver),
    ///   Box::new(DefaultResolver),
    /// ]);
    /// ```
    pub fn new(resolvers: Vec<Box<dyn AssetResolver>>) -> Self {
        Self { resolvers }
    }
}

impl AssetResolver for ResolverChain {

    /// Attempt to resolve a path using the resolvers in the chain.
    /// 
    /// # Example
    /// ```
    /// # use asset_resolver::*;
    /// let resolver = ResolverChain::new(vec![
    ///   Box::new(NullResolver),
    ///   Box::new(DefaultResolver),
    /// ]);
    /// 
    /// let path = resolver.resolve("/home/user/assets/test.png").unwrap();
    /// assert_eq!(path.to_str().unwrap(), "/home/user/assets/test.png");
    /// ```
    fn resolve(&self, path: &str) -> Option<PathBuf> {
        for resolver in &self.resolvers {
            if let Some(path) = resolver.resolve(path) {
                return Some(path);
            }
        }
        None
    }
}


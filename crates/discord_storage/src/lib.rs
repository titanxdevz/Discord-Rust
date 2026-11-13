//! Discord Local Storage & Windows DPAPI Credential Store

pub mod crypto;
pub mod cache;

pub use crypto::{TokenStore, StorageCryptoError};
pub use cache::CacheStore;

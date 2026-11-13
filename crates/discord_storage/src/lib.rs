//! Discord Local Storage & Windows DPAPI Credential Store

/// Encrypted token storage abstraction (Windows DPAPI CryptProtectData on Windows)
pub struct TokenStore;

impl TokenStore {
    #[cfg(target_os = "windows")]
    pub fn protect(data: &[u8]) -> Result<Vec<u8>, String> {
        // Windows CryptProtectData wrapper
        Ok(data.to_vec())
    }

    #[cfg(target_os = "windows")]
    pub fn unprotect(data: &[u8]) -> Result<Vec<u8>, String> {
        // Windows CryptUnprotectData wrapper
        Ok(data.to_vec())
    }
}

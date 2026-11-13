//! Windows DPAPI Encrypted Credential Protection & Cross-Platform Secure Storage

use thiserror::Error;

#[derive(Debug, Error)]
pub enum StorageCryptoError {
    #[error("Windows DPAPI error: {0}")]
    Dpapi(String),
    #[error("UTF-8 decoding error: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
}

/// Token and credential storage encryption
pub struct TokenStore;

impl TokenStore {
    /// Protect a plaintext string using Windows DPAPI (CryptProtectData)
    #[cfg(target_os = "windows")]
    pub fn protect_string(plaintext: &str) -> Result<Vec<u8>, StorageCryptoError> {
        Self::protect_bytes(plaintext.as_bytes())
    }

    /// Unprotect ciphertext bytes using Windows DPAPI (CryptUnprotectData)
    #[cfg(target_os = "windows")]
    pub fn unprotect_string(ciphertext: &[u8]) -> Result<String, StorageCryptoError> {
        let bytes = Self::unprotect_bytes(ciphertext)?;
        Ok(String::from_utf8(bytes)?)
    }

    #[cfg(target_os = "windows")]
    pub fn protect_bytes(data: &[u8]) -> Result<Vec<u8>, StorageCryptoError> {
        unsafe {
            use windows_sys::Win32::Foundation::LocalFree;
            use windows_sys::Win32::Security::Cryptography::{
                CryptProtectData, CRYPT_INTEGER_BLOB,
            };

            let mut in_blob = CRYPT_INTEGER_BLOB {
                cbData: data.len() as u32,
                pbData: data.as_ptr() as *mut u8,
            };
            let mut out_blob = CRYPT_INTEGER_BLOB {
                cbData: 0,
                pbData: std::ptr::null_mut(),
            };

            let ok = CryptProtectData(
                &mut in_blob,
                std::ptr::null(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                &mut out_blob,
            );

            if ok == 0 {
                return Err(StorageCryptoError::Dpapi(
                    "CryptProtectData failed".to_string(),
                ));
            }

            let slice = std::slice::from_raw_parts(out_blob.pbData, out_blob.cbData as usize);
            let protected_bytes = slice.to_vec();
            LocalFree(out_blob.pbData as _);
            Ok(protected_bytes)
        }
    }

    #[cfg(target_os = "windows")]
    pub fn unprotect_bytes(data: &[u8]) -> Result<Vec<u8>, StorageCryptoError> {
        unsafe {
            use windows_sys::Win32::Foundation::LocalFree;
            use windows_sys::Win32::Security::Cryptography::{
                CryptUnprotectData, CRYPT_INTEGER_BLOB,
            };

            let mut in_blob = CRYPT_INTEGER_BLOB {
                cbData: data.len() as u32,
                pbData: data.as_ptr() as *mut u8,
            };
            let mut out_blob = CRYPT_INTEGER_BLOB {
                cbData: 0,
                pbData: std::ptr::null_mut(),
            };

            let ok = CryptUnprotectData(
                &mut in_blob,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null(),
                0,
                &mut out_blob,
            );

            if ok == 0 {
                return Err(StorageCryptoError::Dpapi(
                    "CryptUnprotectData failed".to_string(),
                ));
            }

            let slice = std::slice::from_raw_parts(out_blob.pbData, out_blob.cbData as usize);
            let unprotected_bytes = slice.to_vec();
            LocalFree(out_blob.pbData as _);
            Ok(unprotected_bytes)
        }
    }

    #[cfg(not(target_os = "windows"))]
    pub fn protect_bytes(data: &[u8]) -> Result<Vec<u8>, StorageCryptoError> {
        Ok(data.to_vec())
    }

    #[cfg(not(target_os = "windows"))]
    pub fn unprotect_bytes(data: &[u8]) -> Result<Vec<u8>, StorageCryptoError> {
        Ok(data.to_vec())
    }
}

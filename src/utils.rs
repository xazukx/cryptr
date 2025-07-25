use crate::CryptrError;
use base64::{engine, engine::general_purpose, Engine as _};
use rand::{Rng, TryRngCore};

const B64_STD: engine::GeneralPurpose = general_purpose::STANDARD;

/// Base64 encode the given input
#[inline]
pub fn b64_encode(input: &[u8]) -> String {
    B64_STD.encode(input)
}

/// Base64 decode the given String
#[inline]
pub fn b64_decode(b64: &str) -> Result<Vec<u8>, CryptrError> {
    Ok(B64_STD.decode(b64)?)
}

/// Fills the given buffer with random bytes
#[inline]
pub fn secure_random(buf: &mut [u8]) -> Result<(), CryptrError> {
    rand::rng()
        .try_fill_bytes(buf)
        .map_err(|err| CryptrError::Generic(format!("No access to OsRng: {:?}", err)))?;
    Ok(())
}

/// Returns a random `Vec<u8>` with the specified size
#[inline]
pub fn secure_random_vec(size: usize) -> Result<Vec<u8>, CryptrError> {
    let mut buf = Vec::with_capacity(size);
    (0..size).for_each(|_| buf.push(0));
    secure_random(&mut buf)?;
    Ok(buf)
}

/// Returns a random String with the specified size
#[inline]
pub fn secure_random_alnum(count: usize) -> String {
    rand::rng()
        .sample_iter(rand::distr::Alphabetic)
        .take(count)
        .map(char::from)
        .collect::<String>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_secure_random() {
        let mut buf = [0u8; 32];
        secure_random(&mut buf).unwrap();
        assert_ne!(buf, [0u8; 32]);
        assert_eq!(buf.len(), 32);

        let mut buf = [0u8; 1337];
        secure_random(&mut buf).unwrap();
        assert_ne!(buf, [0u8; 1337]);
        assert_eq!(buf.len(), 1337);
    }

    #[test]
    fn test_secure_random_vec() {
        let rnd = secure_random_vec(13).unwrap();
        assert_ne!(rnd.as_slice(), [0u8; 13]);
        assert_eq!(rnd.len(), 13);

        let rnd = secure_random_vec(32).unwrap();
        assert_ne!(rnd.as_slice(), [0u8; 32]);
        assert_eq!(rnd.len(), 32);
    }
}

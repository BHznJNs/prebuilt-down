use anyhow::{Context, Result};
use sha2::{Digest, Sha256, Sha512};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::{config::HashConfig, traits::to_hex::ToHex, types::hash::HashAlgorithm};

pub fn verify_file(path: &Path, hash: &HashConfig) -> Result<bool> {
    let actual = compute_digest(path, hash.algorithm)?;
    let expected = hash.digest.trim().to_ascii_lowercase();
    return Ok(actual == expected);
}

fn read_chunks(reader: &mut impl Read, mut on_chunk: impl FnMut(&[u8])) -> Result<()> {
    let mut buffer = [0u8; 8192];
    loop {
        let read = reader
            .read(&mut buffer)
            .context("failed to read file for hash")?;
        if read == 0 {
            break;
        }
        on_chunk(&buffer[..read]);
    }
    return Ok(());
}

fn compute_digest(path: &Path, algorithm: HashAlgorithm) -> Result<String> {
    let mut file = File::open(path).context("failed to open file for hash")?;

    match algorithm {
        HashAlgorithm::Sha256 => {
            let mut hasher = Sha256::new();
            read_chunks(&mut file, |chunk| hasher.update(chunk))?;
            let digest = hasher.finalize();
            return Ok(digest.to_hex());
        }
        HashAlgorithm::Sha512 => {
            let mut hasher = Sha512::new();
            read_chunks(&mut file, |chunk| hasher.update(chunk))?;
            let digest = hasher.finalize();
            return Ok(digest.to_hex());
        }
    }
}

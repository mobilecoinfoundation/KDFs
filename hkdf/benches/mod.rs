#![feature(test)]
extern crate test;

use test::Bencher;

type HkdfSha256 = hkdf::Hkdf<sha2::Sha256>;

#[bench]
fn hkdf_sha256_10(b: &mut Bencher) {
    let mut okm = vec![0u8; 10];
    b.iter(|| HkdfSha256::new(Some(&[]), &[]).expand(&[], &mut okm));
    b.bytes = okm.len() as u64;
}

#[bench]
fn hkdf_sha256_1024(b: &mut Bencher) {
    let mut okm = vec![0u8; 1024];
    b.iter(|| HkdfSha256::new(Some(&[]), &[]).expand(&[], &mut okm));
    b.bytes = okm.len() as u64;
}

#[bench]
fn hkdf_sha256_8000(b: &mut Bencher) {
    let mut okm = vec![0u8; 8000];
    b.iter(|| HkdfSha256::new(Some(&[]), &[]).expand(&[], &mut okm));
    b.bytes = okm.len() as u64;
}

type SimpleHkdfSha256 = hkdf::SimpleHkdf<sha2::Sha256>;

#[bench]
fn simple_hkdf_sha256_10(b: &mut Bencher) {
    let mut okm = vec![0u8; 10];
    b.iter(|| SimpleHkdfSha256::new(Some(&[]), &[]).expand(&[], &mut okm));
    b.bytes = okm.len() as u64;
}

#[bench]
fn simple_hkdf_sha256_1024(b: &mut Bencher) {
    let mut okm = vec![0u8; 1024];
    b.iter(|| SimpleHkdfSha256::new(Some(&[]), &[]).expand(&[], &mut okm));
    b.bytes = okm.len() as u64;
}

#[bench]
fn simple_hkdf_sha256_8000(b: &mut Bencher) {
    let mut okm = vec![0u8; 8000];
    b.iter(|| SimpleHkdfSha256::new(Some(&[]), &[]).expand(&[], &mut okm));
    b.bytes = okm.len() as u64;
}

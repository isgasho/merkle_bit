#[cfg(not(any(feature = "use_blake2b", feature = "use_groestl", feature = "use_sha2", feature = "use_sha3", feature = "use_keccak")))]
pub mod default;
#[cfg(feature = "use_blake2b")] pub mod blake2b;
#[cfg(feature = "use_groestl")] pub mod groestl;
#[cfg(feature = "use_sha2")] pub mod sha256;
#[cfg(feature = "use_sha3")] pub mod sha3;
#[cfg(feature = "use_keccak")] pub mod keccak;

#[cfg(not(any(feature = "use_blake2b", feature = "use_groestl", feature = "use_sha2", feature = "use_sha3", feature = "use_keccak")))]
pub type TreeHasher = std::collections::hash_map::DefaultHasher;
#[cfg(not(any(feature = "use_blake2b")))]
pub type TreeHashResult = Vec<u8>;

#[cfg(feature = "use_blake2b")] pub type TreeHasher = crate::tree_hasher::blake2b::Blake2bHasher;
#[cfg(feature = "use_blake2b")] pub type TreeHashResult = crate::tree_hasher::blake2b::Blake2bHashResult;

#[cfg(feature = "use_groestl")] pub type TreeHasher = crate::tree_hasher::groestl::GroestlHasher;
#[cfg(feature = "use_sha2")] pub type TreeHasher = crate::tree_hasher::sha256::Sha256Hasher;
#[cfg(feature = "use_sha3")] pub type TreeHasher = crate::tree_hasher::sha3::Sha3Hasher;
#[cfg(feature = "use_keccak")] pub type TreeHasher = crate::tree_hasher::keccak::KeccakHasher;
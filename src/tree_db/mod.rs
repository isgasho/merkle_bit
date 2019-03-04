#[cfg(not(any(feature = "use_hashbrown")))] pub mod hashmap;
#[cfg(feature = "use_hashbrown")] pub mod hashbrown;

#[cfg(not(any(feature = "use_hashbrown")))]
pub type HashTreeDB = crate::tree_db::hashmap::HashDB;
#[cfg(feature = "use_hashbrown")]
pub type HashTreeDB = crate::tree_db::hashbrown::HashDB;
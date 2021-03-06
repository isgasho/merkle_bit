use std::path::PathBuf;

use crate::merkle_bit::{BinaryMerkleTreeResult, MerkleBIT};
use crate::traits::{Database, Decode, Encode};
use crate::tree_db::rocksdb::RocksDB;
use crate::tree::tree_branch::TreeBranch;
use crate::tree::tree_leaf::TreeLeaf;
use crate::tree::tree_data::TreeData;
use crate::tree::tree_node::TreeNode;
use crate::tree_hasher::{TreeHasher, TreeHashResult};

pub struct RocksTree<ValueType>
    where ValueType: Encode + Decode {
    tree: MerkleBIT<
        RocksDB,
        TreeBranch,
        TreeLeaf,
        TreeData,
        TreeNode,
        TreeHasher,
        TreeHashResult,
        ValueType>
}

impl<ValueType> RocksTree<ValueType>
    where ValueType: Encode + Decode {
    pub fn open(path: &PathBuf, depth: usize) -> BinaryMerkleTreeResult<Self> {
        let db = RocksDB::open(path)?;
        let tree = MerkleBIT::from_db(db, depth)?;
        Ok(RocksTree {
            tree
        })
    }

    pub fn from_db(db: RocksDB, depth: usize) -> BinaryMerkleTreeResult<Self> {
        let tree = MerkleBIT::from_db(db, depth)?;
        Ok(RocksTree {
            tree
        })
    }

    pub fn get(&self, root_hash: &[u8], keys: &mut [&[u8]]) -> BinaryMerkleTreeResult<Vec<Option<ValueType>>> {
        self.tree.get(root_hash, keys)
    }

    pub fn insert(&mut self, previous_root: Option<&[u8]>, keys: &mut [&[u8]], values: &mut [&ValueType]) -> BinaryMerkleTreeResult<Vec<u8>> {
        self.tree.insert(previous_root, keys, values)
    }

    pub fn remove(&mut self, root_hash: &[u8]) -> BinaryMerkleTreeResult<()> {
        self.tree.remove(root_hash)
    }
}
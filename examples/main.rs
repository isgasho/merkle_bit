extern crate starling;

use starling::merkle_bit::BinaryMerkleTreeResult;

fn main() -> BinaryMerkleTreeResult<()> {
    let mut tree = starling::hash_tree::HashTree::new(16)?;

    let key = vec![0x00];
    let value = vec![0x00];

    // Inserting and getting from a tree
    let new_root = tree.insert(None, &mut [&key], &mut vec![&value])?;
    let items = tree.get(&new_root, &mut [&key])?;
    assert_eq!(items, vec![Some(value)]);

    // Attempting to get from a removed root will yield None
    tree.remove(&new_root)?;
    let items2 = tree.get(&new_root, &mut [&key])?;
    assert_eq!(items2, vec![None]);
    Ok(())
}

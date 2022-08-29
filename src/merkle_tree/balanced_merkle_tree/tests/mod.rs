use std::time::Instant;

use crate::merkle_tree::{balanced_merkle_tree::BalancedMerkleTree, AbstractMerkleTree};

#[test]
fn test_balanced_merkle_tree_generate_tree_root() {
     let mock_trees_roots = [
        "bc36789e7a1e281436464229828f817d6612f7b477d66591ff96a9e064bcc98a",
        "b2521d64679bc4720dabfbae7ce17947a5d373d987d3b0cc1e3042ba2054da4a",
        "d359d2743bb3a93ded4c902716931497ae3080f478c14e7af96344a92e9ddd51",
        "fecce4ac8ed6fc57f4d880d6af2b443418d564df8f5d52c6782e952564ed79eb",
        "11aeafa56c9b34805cc86b1c320c9331672c07e600f0a44317051cfa05a0c296",
        "ce24ba488022147ace7a2a962b481707002c079d7c7ca85b108f7489aaedabba",
        "49b36fbd8a6e3a5ea292f621a38d0afa8ac580c56090a9b0d93e0d06b37d1a89",
        "49c6f5244cba156c2170135c98a77f6fc9b812eb201aefcd3e32c38dfcec711a",
        "f54e6007f25df4d2c75a2ec526e4a635dac09b622497862f6062f9340f25ca81",
        "29da8f3f81c6c9dc74665e28dcbfc1645629746613cccbd76c3f8ccd6b1488ae",
    ];

    for x in 1..=mock_trees_roots.len() {
        let nodes = get_n_nodes(x as u32);
        println!("Running test {x}");
        let tree_root = 
            BalancedMerkleTree::generate_tree_root(nodes).unwrap();
        println!("tree_root {:?}", tree_root);
    }
}

#[test]
fn test_generate_root_2() {
    assert!(BalancedMerkleTree::generate_tree_root(get_n_nodes(0)).is_err());
}

#[test]
fn test_generate_root_3() {
    // resistance test
    let now = Instant::now();
    let longTest = get_n_nodes(1_000_000);
    println!("Number of nodes: {}", longTest.len());
    println!("Creating leaves: {}", now.elapsed().as_millis());

    let now = Instant::now();
    assert!(!BalancedMerkleTree::generate_tree_root(longTest).is_err());
    println!("Building tree: {}", now.elapsed().as_millis());
}

fn get_n_nodes(n: u32) -> Vec<Vec<u8>> {
    let mut nodes: Vec<Vec<u8>> = vec![];
    for number in 0..n {
        nodes.push(vec![(number % 256) as u8]);
    }
    return nodes;
}

fn to_array(_size: u8, bytes: &[u8]) -> [u8; 32] {
    let mut array = [0; 32];
    let bytes = &bytes[..];
    array.copy_from_slice(bytes);
    array
}